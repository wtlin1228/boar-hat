package raft

// RPC calls should be idempotent
// - network loss (timeout)
// - network delay (term mismatch)
// - success

import (
	"bytes"
	"fmt"
	"log"
	"math/rand"
	"slices"
	"sync"
	"sync/atomic"
	"time"

	"6.5840/labgob"
	"6.5840/labrpc"
	"6.5840/raftapi"
	tester "6.5840/tester1"
)

const (
	ElectionTimeout   time.Duration = 300 * time.Millisecond
	HeartbeatInterval time.Duration = 100 * time.Millisecond
	ApplyLogInterval  time.Duration = 10 * time.Millisecond
)

const (
	NoVote = -1
)

type State int

const (
	Leader State = iota
	Follower
	Candidate
)

func (s State) String() string {
	switch s {
	case Leader:
		return "Leader"
	case Follower:
		return "Follower"
	case Candidate:
		return "Candidate"
	default:
		return "Unknown"
	}
}

type LogEntry struct {
	Term    int
	Command any
}

// A Go object implementing a single Raft peer.
type Raft struct {
	mu        sync.Mutex          // Lock to protect shared access to this peer's state
	peers     []*labrpc.ClientEnd // RPC end points of all peers
	persister *tester.Persister   // Object to hold this peer's persisted state
	me        int                 // this peer's index into peers[]
	dead      int32               // set by Kill()

	// Your data here (3A, 3B, 3C).
	// Look at the paper's Figure 2 for a description of what
	// state a Raft server must maintain.

	// Persistent state
	currentTerm               int
	votedFor                  int
	log                       []LogEntry
	snapshotLastIncludedIndex int
	snapshotLastIncludedTerm  int
	snapshot                  []byte

	// Volatile state
	state           State
	commitIndex     int
	lastApplied     int
	nextIndex       []int
	matchIndex      []int
	lastSyncLogAt   time.Time // for throttling the AppendEntries and InstallSnapshot RPC calls
	lastHeartbeatAt time.Time // updated when an Heartbeat RPC is received
	applyCh         chan raftapi.ApplyMsg
	applyChMu       sync.Mutex
}

func (rf *Raft) fatalf(format string, a ...interface{}) {
	log.Fatalf("[Raft_%d] term:%d %-9s | %s\n", rf.me, rf.currentTerm, rf.state, fmt.Sprintf(format, a...))
}

func (rf *Raft) debug(format string, a ...interface{}) {
	DPrintf("[Raft_%d] term:%d %-9s | %s", rf.me, rf.currentTerm, rf.state, fmt.Sprintf(format, a...))
}

func (rf *Raft) debugWithLock(format string, a ...interface{}) {
	if Debug {
		rf.mu.Lock()
		rf.debug(format, a...)
		rf.mu.Unlock()
	}
}

func (rf *Raft) getLogEntry(logIndex int) *LogEntry {
	if logIndex == 0 {
		return &rf.log[0]
	}

	if logIndex <= rf.snapshotLastIncludedIndex {
		rf.fatalf("log has been trimmed, logIndex=%d, rf.snapshotLastIncludedIndex=%d", logIndex, rf.snapshotLastIncludedIndex)
	}

	if logIndex > rf.snapshotLastIncludedIndex+len(rf.log)-1 {
		rf.fatalf("log doesn't exist, logIndex=%d, log count=%d", logIndex, rf.snapshotLastIncludedIndex+len(rf.log)-1)
	}

	return &rf.log[logIndex-rf.snapshotLastIncludedIndex]
}

func (rf *Raft) getLogEntryTerm(logIndex int) (term int) {
	if logIndex == 0 {
		return rf.log[0].Term
	}

	if logIndex == rf.snapshotLastIncludedIndex {
		return rf.snapshotLastIncludedTerm
	}

	return rf.getLogEntry(logIndex).Term
}

func (rf *Raft) getLastLogEntryIndexAndTerm() (index int, term int) {
	if rf.snapshotLastIncludedIndex == 0 {
		// not trimmed yet, return the last one
		return len(rf.log) - 1, rf.log[len(rf.log)-1].Term
	}

	if len(rf.log) == 1 {
		// no new log entry after trimmed, return snapshot's last included one
		return rf.snapshotLastIncludedIndex, rf.snapshotLastIncludedTerm
	}

	// new log entries after trimmed, return the last one
	return rf.snapshotLastIncludedIndex + len(rf.log) - 1, rf.log[len(rf.log)-1].Term
}

func (rf *Raft) appendLogEntry(logEntry LogEntry) {
	rf.debug("append #%d log entry, %+v", rf.snapshotLastIncludedIndex+len(rf.log), logEntry)
	rf.log = append(rf.log, logEntry)
}

func (rf *Raft) replaceLogEntry(logIndex int, logEntry LogEntry) {
	rf.debug("replace #%d log entry, %+v -> %+v", logIndex, rf.getLogEntry(logIndex), logEntry)
	rf.log[logIndex-rf.snapshotLastIncludedIndex] = logEntry
}

func (rf *Raft) submitNoOpCommand() {
	if rf.state != Leader {
		rf.fatalf("can only submit command to a leader, command=NoOp")
	}
	rf.debug("submit a NoOp command")
	rf.appendLogEntry(LogEntry{rf.currentTerm, nil})
}

func (rf *Raft) incrementTerm() {
	rf.debug("increment current term, %d -> %d", rf.currentTerm, rf.currentTerm+1)
	rf.currentTerm = rf.currentTerm + 1
}

func (rf *Raft) catchUpTerm(term int) {
	if rf.currentTerm >= term {
		rf.fatalf("no need to catch up term, term=%d", term)
	}
	rf.debug("catch up term, %d -> %d", rf.currentTerm, term)
	rf.currentTerm = term
}

func (rf *Raft) voteFor(server int) {
	rf.debug("vote for server_%d", server)
	rf.votedFor = server
}

func (rf *Raft) changeState(state State) {
	if rf.state == state {
		rf.fatalf("no need to change state, state=%s", state)
	}
	rf.debug("change state, %s -> %s", rf.state, state)
	rf.state = state
}

func (rf *Raft) initNextIndex() {
	index, _ := rf.getLastLogEntryIndexAndTerm()
	rf.debug("initialize next index to %d", index+1)
	for i := range len(rf.peers) {
		rf.nextIndex[i] = index + 1
	}
}

func (rf *Raft) updateNextIndex(server int, index int) {
	rf.debug("update nextIndex[%d], %d -> %d", server, rf.nextIndex[server], index)
	rf.nextIndex[server] = index
}

func (rf *Raft) initMatchIndex() {
	rf.debug("initialize match index to 0")
	for i := range len(rf.peers) {
		rf.matchIndex[i] = 0
	}
}

func (rf *Raft) updateMatchIndex(server int, index int) {
	rf.debug("update matchIndex[%d], %d -> %d", server, rf.matchIndex[server], index)
	rf.matchIndex[server] = index
}

func (rf *Raft) increaseCommitIndex(index int) {
	if index <= rf.commitIndex {
		rf.fatalf("commit index can only increase monotonically, index=%d, rf.commitIndex=%d", index, rf.commitIndex)
	}
	rf.debug("increase commit index, %d -> %d", rf.commitIndex, index)
	rf.commitIndex = index
}

func (rf *Raft) commitIfPossible() {
	matchIndex := slices.Clone(rf.matchIndex)
	slices.Sort(matchIndex)
	l := len(matchIndex)
	commitIndex := matchIndex[l-l/2]
	if commitIndex > rf.commitIndex {
		rf.increaseCommitIndex(commitIndex)
	}
}

func (rf *Raft) increaseLastApplied(index int) {
	if index <= rf.lastApplied {
		rf.fatalf("last applied can only increase monotonically, index=%d, rf.lastApplied=%d", index, rf.lastApplied)
	}
	rf.debug("increase last applied, %d -> %d", rf.lastApplied, index)
	rf.lastApplied = index
}

func (rf *Raft) updateSnapshot(lastIncludedIndex int, lastIncludedTerm int, snapshot []byte) {
	if rf.snapshot != nil && lastIncludedIndex <= rf.snapshotLastIncludedIndex {
		rf.fatalf("no need to update snapshot, snapshot.LastIncludedIndex=%d, rf.snapshotLastIncludedIndex=%d", lastIncludedIndex, rf.snapshotLastIncludedIndex)
	}
	rf.debug("update snapshot, lastIncludedIndex: %d -> %d", rf.snapshotLastIncludedIndex, lastIncludedIndex)
	// always keep the first log entry, it's a placeholder to make it 1-based for raft log
	newLog := []LogEntry{rf.log[0]}
	// if snapshot contains more log entries than the current rf.log does, the trimmed log will be empty
	lastIndex, _ := rf.getLastLogEntryIndexAndTerm()
	if lastIndex > lastIncludedIndex {
		newLog = append(newLog, rf.log[lastIncludedIndex-rf.snapshotLastIncludedIndex+1:]...)
	}
	rf.log = newLog
	rf.snapshotLastIncludedIndex = lastIncludedIndex
	rf.snapshotLastIncludedTerm = lastIncludedTerm
	rf.snapshot = snapshot
}

func (rf *Raft) updateLastHeartbeatAt() {
	rf.lastHeartbeatAt = time.Now()
}

func (rf *Raft) stepDownAsFollower(term int) {
	rf.debug("step down as follower")
	rf.catchUpTerm(term)
	rf.voteFor(NoVote)
	if rf.state != Follower {
		rf.changeState(Follower)
	}
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	return rf.currentTerm, rf.state == Leader
}

// save Raft's persistent state to stable storage,
// where it can later be retrieved after a crash and restart.
// see paper's Figure 2 for a description of what should be persistent.
// before you've implemented snapshots, you should pass nil as the
// second argument to persister.Save().
// after you've implemented snapshots, pass the current snapshot
// (or nil if there's not yet a snapshot).
func (rf *Raft) persist() {
	rf.debug("persist, currentTerm=%d, votedFor=%d, log=%d, snapshotLastIncludedIndex=%d, snapshotLastIncludedTerm=%d",
		rf.currentTerm, rf.votedFor, rf.log, rf.snapshotLastIncludedIndex, rf.snapshotLastIncludedTerm)
	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	e.Encode(rf.currentTerm)
	e.Encode(rf.votedFor)
	e.Encode(rf.log)
	e.Encode(rf.snapshotLastIncludedIndex)
	e.Encode(rf.snapshotLastIncludedTerm)
	raftstate := w.Bytes()
	if rf.snapshot != nil {
		rf.persister.Save(raftstate, rf.snapshot)
	} else {
		rf.persister.Save(raftstate, nil)
	}
}

// restore previously persisted state.
func (rf *Raft) readPersist(data []byte) {
	if len(data) < 1 { // bootstrap without any state?
		return
	}
	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	var currentTerm int
	var votedFor int
	var rfLog []LogEntry
	var lastIncludedIndex int
	var lastIncludedTerm int
	if d.Decode(&currentTerm) != nil ||
		d.Decode(&votedFor) != nil ||
		d.Decode(&rfLog) != nil ||
		d.Decode(&lastIncludedIndex) != nil ||
		d.Decode(&lastIncludedTerm) != nil {
		log.Fatalln("fail to decode the persist data")
	} else {
		rf.currentTerm = currentTerm
		rf.votedFor = votedFor
		rf.log = rfLog
		rf.snapshotLastIncludedIndex = lastIncludedIndex
		rf.snapshotLastIncludedTerm = lastIncludedTerm
		rf.lastApplied = lastIncludedIndex
		rf.commitIndex = lastIncludedIndex
		if lastIncludedIndex != 0 {
			rf.snapshot = rf.persister.ReadSnapshot()
			if len(rf.snapshot) < 1 {
				log.Fatalln("fail to read the snapshot")
			}
		}
	}
}

// how many bytes in Raft's persisted log?
func (rf *Raft) PersistBytes() int {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	return rf.persister.RaftStateSize()
}

// the service says it has created a snapshot that has
// all info up to and including index. this means the
// service no longer needs the log through (and including)
// that index. Raft should now trim its log as much as possible.
func (rf *Raft) Snapshot(index int, snapshot []byte) {
	go func() {
		rf.mu.Lock()
		defer rf.mu.Unlock()

		rf.debug("Snapshot(%d)", index)

		if index <= rf.snapshotLastIncludedIndex {
			rf.debug("No need to do snapshot, snapshot.lastIncludedIndex=%d", rf.snapshotLastIncludedIndex)
			return
		}

		logEntry := rf.getLogEntry(index)
		rf.updateSnapshot(index, logEntry.Term, snapshot)

		rf.persist()
	}()
}

type RequestVoteArgs struct {
	Term         int // candidate's term
	CandidateId  int // candidate requesting vote
	LastLogIndex int // index of candidate's last log entry (§5.4)
	LastLogTerm  int // term of candidate's last log entry (§5.4)
}

type RequestVoteReply struct {
	Term        int  // currentTerm, for candidate to update itself
	VoteGranted bool // true means candidate received vote
}

func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.debug("handle RequestVote, args=%+v", args)

	if args.Term <= rf.currentTerm {
		reply.Term = rf.currentTerm
		reply.VoteGranted = false
		return
	}

	rf.catchUpTerm(args.Term)
	if rf.state != Follower {
		rf.changeState(Follower)
	}

	lastLogIndex, lastLogTerm := rf.getLastLogEntryIndexAndTerm()
	isMyLogMoreUpToDate :=
		// If the logs have last entries with different terms,
		// then the log with the later term is more up-to-date.
		args.LastLogTerm < lastLogTerm ||
			// If the logs end with the same term, then whichever
			// log is longer is more up-to-date.
			(args.LastLogTerm == lastLogTerm && args.LastLogIndex < lastLogIndex)

	if isMyLogMoreUpToDate {
		rf.voteFor(NoVote)
		reply.VoteGranted = false
	} else {
		rf.updateLastHeartbeatAt()
		rf.voteFor(args.CandidateId)
		reply.VoteGranted = true
	}

	rf.persist()
}

// example code to send a RequestVote RPC to a server.
// server is the index of the target server in rf.peers[].
// expects RPC arguments in args.
// fills in *reply with RPC reply, so caller should
// pass &reply.
// the types of the args and reply passed to Call() must be
// the same as the types of the arguments declared in the
// handler function (including whether they are pointers).
//
// The labrpc package simulates a lossy network, in which servers
// may be unreachable, and in which requests and replies may be lost.
// Call() sends a request and waits for a reply. If a reply arrives
// within a timeout interval, Call() returns true; otherwise
// Call() returns false. Thus Call() may not return for a while.
// A false return can be caused by a dead server, a live server that
// can't be reached, a lost request, or a lost reply.
//
// Call() is guaranteed to return (perhaps after a delay) *except* if the
// handler function on the server side does not return.  Thus there
// is no need to implement your own timeouts around Call().
//
// look at the comments in ../labrpc/labrpc.go for more details.
//
// if you're having trouble getting RPC to work, check that you've
// capitalized all field names in structs passed over RPC, and
// that the caller passes the address of the reply struct with &, not
// the struct itself.
func (rf *Raft) sendRequestVote(server int, args *RequestVoteArgs, reply *RequestVoteReply) bool {
	rf.debugWithLock("--- RequestVote ---> %d, term=%d, lastLogIndex=%d, lastLogTerm=%d",
		server, args.Term, args.LastLogIndex, args.LastLogTerm)

	ok := rf.peers[server].Call("Raft.RequestVote", args, reply)

	rf.debugWithLock("<-- RequestVote ---- %d, term=%d, lastLogIndex=%d, lastLogTerm=%d, reply=%+v",
		server, args.Term, args.LastLogIndex, args.LastLogTerm, reply)

	return ok
}

type AppendEntriesArgs struct {
	Term         int        // leader's term
	LeaderId     int        // so follower can redirect clients
	PrevLogIndex int        // index of log entry immediately preceding new ones
	PrevLogTerm  int        // term of prevLogIndex entry
	Entries      []LogEntry // log entries to store (may send more than one for efficiency)
	LeaderCommit int        // leader's commitIndex
}

type AppendEntriesReply struct {
	Term    int  // currentTerm, for leader to update itself
	Success bool // true if follower contained entry matching prevLogIndex and prevLogTerm
	XIndex  int  // faster backup: index of the first entry within XTerm
	XTerm   int
}

func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.debug("handle AppendEntries, args=%+v", args)

	if args.Term < rf.currentTerm {
		reply.Term = rf.currentTerm
		reply.Success = false
		return
	}

	if args.Term > rf.currentTerm {
		rf.voteFor(NoVote)
		rf.catchUpTerm(args.Term)
	}

	rf.updateLastHeartbeatAt()
	if rf.state != Follower {
		rf.changeState(Follower)
	}

	if args.PrevLogIndex > rf.snapshotLastIncludedIndex+len(rf.log)-1 {
		lastIndex, lastTerm := rf.getLastLogEntryIndexAndTerm()
		rf.debug("    case 1: args.PrevLogIndex %d is greater than the last log index %d", args.PrevLogIndex, lastIndex)
		xIndex := lastIndex
		xTerm := lastTerm
		for i := rf.snapshotLastIncludedIndex + len(rf.log) - 1; i >= rf.snapshotLastIncludedIndex; i-- {
			if rf.getLogEntryTerm(i) != xTerm {
				break
			}
			xIndex = i
		}
		reply.Success = false
		reply.XIndex = xIndex
		reply.XTerm = xTerm
	} else if args.PrevLogIndex < rf.snapshotLastIncludedIndex {
		rf.debug("    case 2: args.PrevLogIndex %d is smaller than the snapshot.LastIncludedIndex %d", args.PrevLogIndex, rf.snapshotLastIncludedIndex)
		reply.Success = false
		reply.XIndex = rf.snapshotLastIncludedIndex
		reply.XTerm = rf.snapshotLastIncludedTerm
	} else if args.PrevLogTerm != rf.getLogEntryTerm(args.PrevLogIndex) {
		// first log entry {term: 0, command: nil} guarantees that args.PrevLogIndex - 1 >= 0
		rf.debug("    case 3: args.PrevLogTerm %d isn't equal to rf.getLogEntryTerm(%d) %d", args.PrevLogTerm, args.PrevLogIndex, rf.getLogEntryTerm(args.PrevLogIndex))
		xIndex := args.PrevLogIndex - 1
		xTerm := rf.getLogEntryTerm(xIndex)
		for i := xIndex; i >= rf.snapshotLastIncludedIndex; i-- {
			if rf.getLogEntryTerm(i) != xTerm {
				break
			}
			xIndex = i
		}
		reply.Success = false
		reply.XIndex = xIndex
		reply.XTerm = xTerm
	} else {
		rf.debug("    case 4: start updating log")
		maybeNeedCleanUp := false
		lastReplacedIndex := -1
		lastIndex, _ := rf.getLastLogEntryIndexAndTerm()
		for i := range len(args.Entries) {
			logIndex := args.PrevLogIndex + 1 + i
			if logIndex > lastIndex {
				rf.appendLogEntry(args.Entries[i])
				maybeNeedCleanUp = false
			} else if rf.getLogEntry(logIndex).Term != args.Entries[i].Term {
				rf.replaceLogEntry(logIndex, args.Entries[i])
				lastReplacedIndex = logIndex
				maybeNeedCleanUp = true
			}
		}
		if maybeNeedCleanUp {
			lastIndex, _ := rf.getLastLogEntryIndexAndTerm()
			for i := lastReplacedIndex + 1; i <= lastIndex; i++ {
				logEntry := rf.getLogEntry(i)
				if logEntry.Term < args.Entries[len(args.Entries)-1].Term {
					rf.debug("the term of #%d entry %+v is smaller than the term of #%d entry (last replaced entry) %+v",
						i, logEntry, lastReplacedIndex, args.Entries[len(args.Entries)-1])
					rf.debug("do clean up, log %+v -> log %+v", rf.log, rf.log[:i])
					rf.log = rf.log[:i]
					break
				}
			}
		}
		if args.LeaderCommit > rf.commitIndex {
			rf.increaseCommitIndex(args.LeaderCommit)
		}
		reply.Success = true
	}

	rf.persist()
}

func (rf *Raft) sendAppendEntries(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	rf.debugWithLock("--- AppendEntries ---> %d, term=%d, PrevLogIndex=%d, PrevLogTerm=%d, LeaderCommit=%d, Entries=%+v",
		server, args.Term, args.PrevLogIndex, args.PrevLogTerm, args.LeaderCommit, args.Entries)

	ok := rf.peers[server].Call("Raft.AppendEntries", args, reply)

	rf.debugWithLock("<-- AppendEntries ---- %d, term=%d, PrevLogIndex=%d, PrevLogTerm=%d, LeaderCommit=%d, Entries=%+v, reply=%+v",
		server, args.Term, args.PrevLogIndex, args.PrevLogTerm, args.LeaderCommit, args.Entries, reply)

	return ok
}

type InstallSnapshotArgs struct {
	Term              int    // leader's term
	LeaderId          int    // so follower can redirect clients
	LastIncludedIndex int    // the snapshot replaces all entries up through and including this index
	LastIncludedTerm  int    // term of lastIncludedIndex
	Offset            int    // byte offset where chunk is positioned in the snapshot file
	Data              []byte // raw bytes of the snapshot chunk, starting at offset
	Done              bool   // true if this is the last chunk
}

type InstallSnapshotReply struct {
	Term    int // currentTerm, for leader to update itself
	Success bool
}

func (rf *Raft) InstallSnapshot(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.debug("handle InstallSnapshot, args=%+v", args)

	if args.Term < rf.currentTerm {
		reply.Term = rf.currentTerm
		reply.Success = false
		return
	}

	if args.Term > rf.currentTerm {
		rf.voteFor(NoVote)
		rf.catchUpTerm(args.Term)
	}

	rf.updateLastHeartbeatAt()
	if rf.state != Follower {
		rf.changeState(Follower)
	}

	if rf.snapshot == nil || rf.snapshotLastIncludedIndex < args.LastIncludedIndex {
		rf.updateSnapshot(args.LastIncludedIndex, args.LastIncludedTerm, args.Data)
		// don't need to apply snapshot if all log entries in the snapshot have been applied
		if !rf.killed() && rf.lastApplied < rf.snapshotLastIncludedIndex {
			rf.debug("apply snapshot, snapshotLastIncludedIndex=%d, snapshotLastIncludedTerm=%d",
				rf.snapshotLastIncludedIndex, rf.snapshotLastIncludedTerm)
			rf.applyChMu.Lock()
			rf.applyCh <- raftapi.ApplyMsg{
				SnapshotValid: true,
				Snapshot:      rf.snapshot,
				SnapshotTerm:  rf.snapshotLastIncludedTerm,
				SnapshotIndex: rf.snapshotLastIncludedIndex,
			}
			rf.applyChMu.Unlock()
			if rf.snapshotLastIncludedIndex > rf.lastApplied {
				rf.increaseLastApplied(args.LastIncludedIndex)
			}
			if rf.snapshotLastIncludedIndex > rf.commitIndex {
				rf.increaseCommitIndex(args.LastIncludedIndex)
			}
		}
	}

	reply.Success = true

	rf.persist()
}

func (rf *Raft) sendInstallSnapshot(server int, args *InstallSnapshotArgs, reply *InstallSnapshotReply) bool {
	rf.debugWithLock("--- InstallSnapshot ---> %d, term=%d, LastIncludedIndex=%d, LastIncludedTerm=%d",
		server, args.Term, args.LastIncludedIndex, args.LastIncludedTerm)

	ok := rf.peers[server].Call("Raft.InstallSnapshot", args, reply)

	rf.debugWithLock("<-- InstallSnapshot ---- %d, term=%d, LastIncludedIndex=%d, LastIncludedTerm=%d, reply=%+v",
		server, args.Term, args.LastIncludedIndex, args.LastIncludedTerm, reply)

	return ok
}

// the service using Raft (e.g. a k/v server) wants to start
// agreement on the next command to be appended to Raft's log. if this
// server isn't the leader, returns false. otherwise start the
// agreement and return immediately. there is no guarantee that this
// command will ever be committed to the Raft log, since the leader
// may fail or lose an election. even if the Raft instance has been killed,
// this function should return gracefully.
//
// the first return value is the index that the command will appear at
// if it's ever committed. the second return value is the current
// term. the third return value is true if this server believes it is
// the leader.
func (rf *Raft) Start(command interface{}) (index int, term int, isLeader bool) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.state != Leader {
		return -1, -1, false
	}

	rf.appendLogEntry(LogEntry{
		Term:    rf.currentTerm,
		Command: command,
	})
	rf.persist()

	lastIndex, lastTerm := rf.getLastLogEntryIndexAndTerm()

	go rf.startSyncLog()

	return lastIndex, lastTerm, true
}

// the tester doesn't halt goroutines created by Raft after each test,
// but it does call the Kill() method. your code can use killed() to
// check whether Kill() has been called. the use of atomic avoids the
// need for a lock.
//
// the issue is that long-running goroutines use memory and may chew
// up CPU time, perhaps causing later tests to fail and generating
// confusing debug output. any goroutine with a long-running loop
// should call killed() to check whether it should stop.
func (rf *Raft) Kill() {
	rf.debugWithLock("Kill")

	atomic.StoreInt32(&rf.dead, 1)

	rf.applyChMu.Lock()
	close(rf.applyCh)
	rf.applyChMu.Unlock()
}

func (rf *Raft) killed() bool {
	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

func (rf *Raft) startElection(args RequestVoteArgs) {
	rf.debugWithLock("start election for term %d", args.Term)

	done := make(chan bool)

	winElection := func() {
		rf.mu.Lock()
		defer rf.mu.Unlock()

		if rf.currentTerm != args.Term || rf.state != Candidate {
			return
		}

		rf.changeState(Leader)
		rf.initMatchIndex()
		rf.initNextIndex()
		// from paper: leader commits a no-op entry at the start of its term
		// but it will fail the 2B test, so let's disable it for now
		// rf.submitNoOpCommand()
		rf.persist()

		go rf.startSyncLog()
	}

	requestVote := func(server int) {
		reply := RequestVoteReply{}
		ok := rf.sendRequestVote(server, &args, &reply)
		if !ok {
			done <- false
			return
		}

		rf.mu.Lock()

		if rf.currentTerm < reply.Term {
			rf.stepDownAsFollower(reply.Term)
			rf.persist()
			rf.mu.Unlock()
			done <- false
			return
		}

		if rf.currentTerm != args.Term {
			rf.mu.Unlock()
			done <- false
			return
		}

		rf.mu.Unlock()
		done <- reply.VoteGranted
	}

	for server := range len(rf.peers) {
		if server != rf.me {
			go requestVote(server)
		}
	}

	count := 1
	finished := false
	for i := 0; i < len(rf.peers)-1; i++ {
		voteGranted := <-done
		if voteGranted {
			count += 1
			if finished == false && count > len(rf.peers)/2 {
				finished = true
				winElection()
			}
		}
	}

	close(done)
}

func (rf *Raft) electionLoop() {
	for rf.killed() == false {
		rf.mu.Lock()
		if rf.state == Follower && rf.lastHeartbeatAt.Add(ElectionTimeout).Before(time.Now()) {
			rf.changeState(Candidate)
		}
		if rf.state == Candidate {
			rf.incrementTerm()
			rf.voteFor(rf.me)
			rf.persist()
			index, term := rf.getLastLogEntryIndexAndTerm()
			go rf.startElection(RequestVoteArgs{
				Term:         rf.currentTerm,
				CandidateId:  rf.me,
				LastLogIndex: index,
				LastLogTerm:  term,
			})
		}
		rf.mu.Unlock()

		// pause for a random amount of time between 50 and 350
		// milliseconds.
		ms := 50 + (rand.Int63() % 300)
		time.Sleep(time.Duration(ms) * time.Millisecond)
	}
}

func (rf *Raft) startSyncLog() {
	syncLogByInstallSnapshot := func(server int, args InstallSnapshotArgs) {
		reply := InstallSnapshotReply{}
		ok := rf.sendInstallSnapshot(server, &args, &reply)
		if !ok {
			return
		}

		rf.mu.Lock()
		defer rf.mu.Unlock()

		if rf.currentTerm < reply.Term {
			rf.stepDownAsFollower(reply.Term)
			rf.persist()
			return
		}

		if rf.currentTerm != args.Term || !reply.Success {
			return
		}

		lastIndex := args.LastIncludedIndex
		lastTerm := args.LastIncludedTerm
		rf.updateNextIndex(server, max(rf.nextIndex[server], lastIndex+1))
		if lastTerm == rf.currentTerm &&
			lastIndex > rf.matchIndex[server] {
			rf.updateMatchIndex(server, lastIndex)
			rf.commitIfPossible()
		}
	}

	syncLogByAppendEntries := func(server int, args AppendEntriesArgs) {
		reply := AppendEntriesReply{}
		ok := rf.sendAppendEntries(server, &args, &reply)
		if !ok {
			return
		}

		rf.mu.Lock()
		defer rf.mu.Unlock()

		if rf.currentTerm < reply.Term {
			rf.stepDownAsFollower(reply.Term)
			rf.persist()
			return
		}

		if rf.currentTerm != args.Term {
			return
		}

		if !reply.Success {
			// TODO: check xTerm and xIndex for even faster backup
			rf.updateNextIndex(server, max(reply.XIndex+1, 1))
			return
		}

		lastIndex := args.PrevLogIndex + len(args.Entries)
		lastTerm := args.PrevLogTerm
		if len(args.Entries) > 0 {
			lastTerm = args.Entries[len(args.Entries)-1].Term
		}
		rf.updateNextIndex(server, max(rf.nextIndex[server], lastIndex+1))
		if lastTerm == rf.currentTerm &&
			lastIndex > rf.matchIndex[server] {
			rf.updateMatchIndex(server, lastIndex)
			rf.commitIfPossible()
		}
	}

	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.state != Leader {
		return
	}

	lastLogEntryIndex, _ := rf.getLastLogEntryIndexAndTerm()

	for server := range len(rf.peers) {
		if server != rf.me {
			if rf.nextIndex[server] <= rf.snapshotLastIncludedIndex {
				go syncLogByInstallSnapshot(server, InstallSnapshotArgs{
					Term:              rf.currentTerm,
					LeaderId:          rf.me,
					LastIncludedIndex: rf.snapshotLastIncludedIndex,
					LastIncludedTerm:  rf.snapshotLastIncludedTerm,
					Offset:            0,
					Data:              rf.snapshot,
					Done:              rf.snapshotLastIncludedIndex == lastLogEntryIndex,
				})
			} else {
				nextIndex := rf.nextIndex[server]
				go syncLogByAppendEntries(server, AppendEntriesArgs{
					Term:         rf.currentTerm,
					LeaderId:     rf.me,
					PrevLogIndex: nextIndex - 1,
					PrevLogTerm:  rf.getLogEntryTerm(nextIndex - 1),
					Entries:      append([]LogEntry{}, rf.log[nextIndex-rf.snapshotLastIncludedIndex:]...),
					LeaderCommit: rf.commitIndex,
				})
			}
		}
	}
}

func (rf *Raft) heartbeatLoop() {
	for rf.killed() == false {
		go rf.startSyncLog()
		time.Sleep(HeartbeatInterval)
	}
}

func (rf *Raft) applyLogEntries() {
	rf.mu.Lock()
	start := rf.lastApplied + 1
	end := rf.commitIndex
	if start <= end {
		rf.debug("applyLogEntries from %d to %d", start, end)
	}
	rf.mu.Unlock()

	for i := start; i <= end; i++ {
		rf.mu.Lock()
		rf.debug("apply #%d log entry, %+v", i, rf.getLogEntry(i))
		command := rf.getLogEntry(i).Command
		rf.mu.Unlock()

		if !rf.killed() && command != nil {
			rf.applyChMu.Lock()
			rf.applyCh <- raftapi.ApplyMsg{
				CommandValid: true,
				Command:      command,
				CommandIndex: i,
			}
			rf.applyChMu.Unlock()
		}

		rf.mu.Lock()
		rf.increaseLastApplied(i)
		rf.mu.Unlock()
	}
}

func (rf *Raft) applyLogLoop() {
	for rf.killed() == false {
		rf.applyLogEntries()
		time.Sleep(ApplyLogInterval)
	}
}

// the service or tester wants to create a Raft server. the ports
// of all the Raft servers (including this one) are in peers[]. this
// server's port is peers[me]. all the servers' peers[] arrays
// have the same order. persister is a place for this server to
// save its persistent state, and also initially holds the most
// recent saved state, if any. applyCh is a channel on which the
// tester or service expects Raft to send ApplyMsg messages.
// Make() must return quickly, so it should start goroutines
// for any long-running work.
func Make(peers []*labrpc.ClientEnd, me int,
	persister *tester.Persister, applyCh chan raftapi.ApplyMsg) raftapi.Raft {
	rf := &Raft{}
	rf.peers = peers
	rf.persister = persister
	rf.me = me

	// Persistent state
	rf.currentTerm = 0
	rf.votedFor = NoVote
	rf.log = []LogEntry{
		{Term: 0, Command: nil},
	}
	rf.snapshotLastIncludedIndex = 0
	rf.snapshotLastIncludedTerm = 0
	rf.snapshot = nil

	// Volatile state
	rf.state = Follower
	rf.commitIndex = 0
	rf.lastApplied = 0
	rf.nextIndex = make([]int, len(peers))
	rf.matchIndex = make([]int, len(peers))
	rf.lastSyncLogAt = time.Now()
	rf.lastHeartbeatAt = time.Now()
	rf.applyCh = applyCh

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	rf.debug("Make Raft, currentTerm=%d, log=%+v, snapshotLastIncludedIndex=%d, snapshotLastIncludedTerm=%d",
		rf.currentTerm, rf.log, rf.snapshotLastIncludedIndex, rf.snapshotLastIncludedTerm)

	go rf.electionLoop()
	go rf.heartbeatLoop()
	go rf.applyLogLoop()

	return rf
}

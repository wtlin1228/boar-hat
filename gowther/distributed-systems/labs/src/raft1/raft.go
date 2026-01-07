package raft

// The file raftapi/raft.go defines the interface that raft must
// expose to servers (or the tester), but see comments below for each
// of these functions for more details.
//
// Make() creates a new raft peer that implements the raft interface.

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

const NoVote = -1

const electionTimeout time.Duration = 500 * time.Millisecond

// The tester requires that the leader send heartbeat RPCs no more than ten times
// per second.
const sendAppendEntriesInterval time.Duration = 200 * time.Millisecond

const applyCommittedEntriesInterval time.Duration = 10 * time.Millisecond

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

	// at any given time each server is in one of three states:
	// leader, follower, or candidate
	state State

	// updated when an AppendEntries RPC is received
	lastHeartbeatAt time.Time

	// latest term the server has seen
	// (initialized to 0, increases monotonically)
	currentTerm int

	// candidate id that received vote in the current term,
	// -1 means hasn't voted for current term
	voteFor int

	// log entries, each entry contains command for state machine,
	// and term when entry was received by leader
	log *RaftLog

	// index of highest log entry known to be committed
	// (initialized to 0, increases monotonically)
	commitIndex int

	// index of highest log entry applied to state machine
	// (initialized to 0, increases monotonically)
	lastApplied int

	// for each server, index of the next log entry to send to that server
	// (initialized to leader last log index + 1)
	nextIndex []int

	// for each server, index of highest log entry known to be replicated on server
	// (initialized to 0, increases monotonically)
	matchIndex []int

	// the snapshot from the service layer
	snapshot []byte

	applyCh chan raftapi.ApplyMsg
}

func (rf *Raft) Debug(label string) {
	DPrintf(`[%d] term:%d %-9s  - %s
	{	
		voteFor:     %d,
		commitIndex: %d,
		lastApplied: %d,
		nextIndex:   %v,
		matchIndex:  %v,
		log.startAt: %d,
		log count:   %d,
		log.data:    %+v
	}
	`, rf.me, rf.currentTerm, rf.state, label,
		rf.voteFor,
		rf.commitIndex,
		rf.lastApplied,
		rf.nextIndex,
		rf.matchIndex,
		rf.log.startAt,
		rf.log.getCount(),
		rf.log.data)
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	// Your code here (3A).
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
	// Your code here (3C).
	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	e.Encode(rf.currentTerm)
	e.Encode(rf.voteFor)
	e.Encode(rf.log.data)
	e.Encode(rf.log.startAt)
	raftstate := w.Bytes()
	if len(rf.snapshot) > 0 {
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
	// Your code here (3C).
	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	var currentTerm int
	var voteFor int
	var logEntries []LogEntry
	var logStartAt int
	if d.Decode(&currentTerm) != nil ||
		d.Decode(&voteFor) != nil ||
		d.Decode(&logEntries) != nil ||
		d.Decode(&logStartAt) != nil {
		log.Fatalln("fail to decode the persist data")
	} else {
		rf.currentTerm = currentTerm
		rf.voteFor = voteFor
		rf.log.data = logEntries

		if logStartAt != 0 {
			rf.log.startAt = logStartAt
			rf.commitIndex = logStartAt
			rf.lastApplied = logStartAt

			snapshot := rf.persister.ReadSnapshot()
			if len(snapshot) == 0 {
				log.Fatalln("fail to read snapshot")
			}
			rf.snapshot = snapshot
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
	rf.Debug(fmt.Sprintf("Snapshot(%d) start", index))

	// Your code here (3D).
	if index <= rf.log.startAt {
		// it has been snapshotted already
		return
	}

	rf.snapshot = snapshot
	rf.Debug(fmt.Sprintf("rf.log.trim(%d) start", index))
	rf.log.trim(index)
	rf.Debug(fmt.Sprintf("rf.log.trim(%d) done", index))
	rf.persist()
}

// example RequestVote RPC arguments structure.
// field names must start with capital letters!
type RequestVoteArgs struct {
	// Your data here (3A, 3B).
	Term         int // candidate's term
	CandidateId  int // candidate requesting vote
	LastLogIndex int // index of candidate's last log entry
	LastLogTerm  int // term of candidate's last log entry
}

// example RequestVote RPC reply structure.
// field names must start with capital letters!
type RequestVoteReply struct {
	// Your data here (3A).
	Term        int  // currentTerm, for candidate to update itself
	VoteGranted bool // true means candidate received vote
}

// example RequestVote RPC handler.
func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.Debug(fmt.Sprintf("RequestVote start, args=%+v", args))

	if rf.currentTerm >= args.Term {
		reply.Term = rf.currentTerm
		reply.VoteGranted = false
		rf.Debug(fmt.Sprintf("RequestVote done, reply=%+v", reply))
		return
	}

	reply.Term = args.Term
	rf.currentTerm = args.Term
	rf.state = Follower

	lastLogIndex := rf.log.getLastLogIndex()
	lastLogEntryTerm := rf.log.getLastLogTerm()

	isMyLogMoreUpToDate :=
		// If the logs have last entries with different terms,
		// then the log with the later term is more up-to-date.
		args.LastLogTerm < lastLogEntryTerm ||
			// If the logs end with the same term, then whichever
			// log is longer is more up-to-date.
			(args.LastLogTerm == lastLogEntryTerm && args.LastLogIndex < lastLogIndex)

	if isMyLogMoreUpToDate {
		rf.voteFor = NoVote
		reply.VoteGranted = false
	} else {
		rf.voteFor = args.CandidateId
		reply.VoteGranted = true
	}

	rf.persist()

	rf.Debug(fmt.Sprintf("RequestVote done, reply=%+v", reply))
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
	DPrintf("[%d] term:%d %-9s  -- vote --> [%d] args=%+v", rf.me, rf.currentTerm, rf.state, server, args)

	ok := rf.peers[server].Call("Raft.RequestVote", args, reply)

	if reply.VoteGranted {
		DPrintf("[%d] term:%d %-9s <-- vote --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	} else {
		DPrintf("[%d] term:%d %-9s x-- vote --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	}

	rf.mu.Lock()
	if rf.currentTerm < reply.Term {
		rf.currentTerm = reply.Term
		rf.voteFor = NoVote
		rf.state = Follower
		rf.persist()
		rf.Debug(fmt.Sprintf("sendRequestVote(%d), update current term", server))
	}
	rf.mu.Unlock()

	return ok
}

type AppendEntriesArgs struct {
	Term         int // leader's term
	LeaderId     int // so follower can redirect clients
	PrevLogIndex int // index of log entry immediately preceding new ones
	PrevLogTerm  int // term of prevLogIndex entry

	// term of prevLogIndex entry log entries to store
	// (empty for heartbeat; may send more than one for efficiency)
	Entries []LogEntry

	LeaderCommit int // leader's commit index
}

type AppendEntriesReply struct {
	Term    int  // currentTerm, for leader to update itself
	Success bool // true if follower contained entry matching prevLogIndex and prevLogTerm
	XIndex  int  // faster backup: index of the first entry within XTerm
}

func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.Debug(fmt.Sprintf("AppendEntries start, args=%+v", args))

	if rf.currentTerm > args.Term {
		reply.Term = rf.currentTerm
		return
	} else if rf.currentTerm < args.Term {
		rf.voteFor = NoVote
		rf.currentTerm = args.Term
	}

	reply.Term = args.Term
	rf.lastHeartbeatAt = time.Now()
	rf.state = Follower

	var prevLogEntry *LogEntry
	var ok bool
	if rf.log.getLastLogIndex() >= args.PrevLogIndex {
		prevLogEntry, ok = rf.log.getLogEntry(args.PrevLogIndex)
		if !ok {
			log.Fatalln("unimplemented!")
		}
	}

	isPrevLogEntryIdentical :=
		rf.log.getLastLogIndex() >= args.PrevLogIndex &&
			prevLogEntry.Term == args.PrevLogTerm

	if isPrevLogEntryIdentical {
		reply.Success = true
		if len(args.Entries) > 0 {
			DPrintf("[%d] term:%d %-9s  - before replaces %d log entries starts from %d, entries=%+v\n    count=%d, startAt=%d, logs=%+v", rf.me, rf.currentTerm, rf.state, len(args.Entries), args.PrevLogIndex+1, args.Entries, rf.log.getCount(), rf.log.startAt, rf.log.data)
			rf.log.replace(args.PrevLogIndex+1, args.Entries)
			DPrintf("[%d] term:%d %-9s  - after replaces %d log entries starts from %d, entries=%+v\n    count=%d, startAt=%d, logs=%+v", rf.me, rf.currentTerm, rf.state, len(args.Entries), args.PrevLogIndex+1, args.Entries, rf.log.getCount(), rf.log.startAt, rf.log.data)
		}
		rf.commitIndex = args.LeaderCommit
	} else {
		reply.Success = false
		reply.XIndex = rf.log.getXIndex(args.PrevLogTerm, args.PrevLogIndex)
	}

	rf.Debug(fmt.Sprintf("AppendEntries done, reply=%+v", reply))

	rf.persist()
}

func (rf *Raft) sendAppendEntries(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	DPrintf("[%d] term:%d %-9s  -- append entries --> [%d] args=%+v", rf.me, rf.currentTerm, rf.state, server, args)

	ok := rf.peers[server].Call("Raft.AppendEntries", args, reply)

	if reply.Success {
		DPrintf("[%d] term:%d %-9s <-- append entries --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	} else {
		DPrintf("[%d] term:%d %-9s x-- append entries --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	}

	rf.mu.Lock()
	if rf.currentTerm < reply.Term {
		rf.currentTerm = reply.Term
		rf.voteFor = NoVote
		rf.state = Follower
		rf.persist()
		rf.Debug(fmt.Sprintf("sendAppendEntries(%d), update current term", server))
	}
	rf.mu.Unlock()

	return ok
}

type InstallSnapshotArgs struct {
	Snapshot      []byte
	SnapshotTerm  int
	SnapshotIndex int
}

type InstallSnapshotReply struct {
	Success bool
}

func (rf *Raft) InstallSnapshot(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.Debug(fmt.Sprintf("InstallSnapshot start, snapshotTerm=%d, snapshotIndex=%d, log.startAt=%d", args.SnapshotTerm, args.SnapshotIndex, rf.log.startAt))

	if rf.log.startAt >= args.SnapshotIndex {
		reply.Success = false
		return
	}

	rf.lastHeartbeatAt = time.Now()

	rf.log.data = []LogEntry{{Term: args.SnapshotTerm, Command: nil}}
	rf.log.startAt = args.SnapshotIndex
	if rf.commitIndex < args.SnapshotIndex {
		rf.commitIndex = args.SnapshotIndex
	}
	if rf.lastApplied < args.SnapshotIndex {
		rf.lastApplied = args.SnapshotIndex
	}
	rf.snapshot = args.Snapshot

	rf.persist()

	var msg raftapi.ApplyMsg
	msg.CommandValid = false
	msg.SnapshotValid = true
	msg.Snapshot = args.Snapshot
	msg.SnapshotTerm = args.SnapshotTerm
	msg.SnapshotIndex = args.SnapshotIndex
	rf.applyCh <- msg

	reply.Success = true
	rf.Debug("InstallSnapshot done")
}

func (rf *Raft) sendInstallSnapshot(server int, args *InstallSnapshotArgs, reply *InstallSnapshotReply) bool {
	DPrintf("[%d] term:%d %-9s  -- install snapshot --> [%d] args=%+v", rf.me, rf.currentTerm, rf.state, server, args)

	ok := rf.peers[server].Call("Raft.InstallSnapshot", args, reply)

	if reply.Success {
		DPrintf("[%d] term:%d %-9s <-- install snapshot --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	} else {
		DPrintf("[%d] term:%d %-9s x-- install snapshot --  [%d] reply=%+v", rf.me, rf.currentTerm, rf.state, server, reply)
	}

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
func (rf *Raft) Start(command interface{}) (int, int, bool) {
	// Your code here (3B).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	term := rf.currentTerm
	isLeader := rf.state == Leader

	if isLeader {
		rf.Debug(fmt.Sprintf("Start(%v) start", command))
		rf.log.appendOne(LogEntry{Term: term, Command: command})
		rf.Debug(fmt.Sprintf("Start(%v) done", command))
		rf.persist()
	}

	index := rf.log.getLastLogIndex()

	return index, term, isLeader
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
	atomic.StoreInt32(&rf.dead, 1)
	// Your code here, if desired.
}

func (rf *Raft) killed() bool {
	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

func (rf *Raft) election(peersCount int, term int, candidateId int, lastLogIndex int, lastLogTerm int) {
	c := make(chan bool)

	for i := range peersCount {
		if i == candidateId {
			continue
		}
		go func() {
			args := RequestVoteArgs{
				Term:         term,
				CandidateId:  candidateId,
				LastLogIndex: lastLogIndex,
				LastLogTerm:  lastLogTerm,
			}
			reply := RequestVoteReply{}
			ok := rf.sendRequestVote(i, &args, &reply)
			if ok && reply.Term == term {
				c <- reply.VoteGranted
			} else {
				c <- false
			}
		}()
	}

	count := 1 // because it votes for itself
	finished := 1

	for {
		vote := <-c
		if vote {
			count += 1
		}
		finished += 1

		rf.mu.Lock()
		isElectionOutdated := rf.state != Candidate || rf.currentTerm != term
		rf.mu.Unlock()

		if isElectionOutdated {
			// this election is outdated, its result doesn't matter anymore
			break
		}

		if count > peersCount/2 {
			DPrintf("[%d] wins the election for term %d with %d votes", rf.me, term, count)
			rf.mu.Lock()
			rf.state = Leader
			for i := range len(rf.peers) {
				rf.nextIndex[i] = rf.log.getLastLogIndex() + 1
				rf.matchIndex[i] = 0
			}
			rf.mu.Unlock()
			// notify peers as soon as possible
			rf.sendAppendEntriesIfNeeded()
			break
		}

		if finished == peersCount {
			// didn't win the election, do nothing
			break
		}
	}
}

func (rf *Raft) startElectionIfNeeded() {
	rf.mu.Lock()

	if rf.state == Follower {
		isTimeout := rf.lastHeartbeatAt.Add(electionTimeout).Before(time.Now())
		if isTimeout {
			rf.state = Candidate
		}
	}

	if rf.state != Candidate {
		rf.mu.Unlock()
		return
	}

	rf.currentTerm += 1
	rf.voteFor = rf.me
	rf.persist()

	peersCount := len(rf.peers)
	term := rf.currentTerm
	candidateId := rf.me
	lastLogIndex := rf.log.getLastLogIndex()
	lastLogTerm := rf.log.getLastLogTerm()

	rf.mu.Unlock()

	go rf.election(peersCount, term, candidateId, lastLogIndex, lastLogTerm)
}

func (rf *Raft) commitIfPossible() {
	matchIndexes := slices.Clone(rf.matchIndex)
	slices.Sort(matchIndexes)
	l := len(matchIndexes)
	commitIndex := matchIndexes[l-l/2]
	// when the leader is just selected, matchIndexes
	// will be reset to 0, but the commitIndex shouldn't
	// decrease
	if commitIndex > rf.commitIndex {
		rf.commitIndex = commitIndex
	}
}

func (rf *Raft) sendAppendEntriesIfNeeded() {
	rf.mu.Lock()

	if rf.state != Leader {
		rf.mu.Unlock()
		return
	}

	term := rf.currentTerm
	me := rf.me
	peersCount := len(rf.peers)

	allInstallSnapshotArgs := make([]*InstallSnapshotArgs, peersCount)
	allAppendEntriesArgs := make([]*AppendEntriesArgs, peersCount)

	for i := range peersCount {
		if i == me {
			continue
		}

		nextIndex := rf.nextIndex[i]

		if nextIndex-1 < rf.log.startAt {
			logEntry, _ := rf.log.getLogEntry(rf.log.startAt)
			allInstallSnapshotArgs[i] = &InstallSnapshotArgs{
				Snapshot:      rf.snapshot,
				SnapshotTerm:  logEntry.Term,
				SnapshotIndex: rf.log.startAt,
			}
		} else {
			prevLogEntry, _ := rf.log.getLogEntry(nextIndex - 1)
			allAppendEntriesArgs[i] = &AppendEntriesArgs{
				Term:         term,
				LeaderId:     me,
				LeaderCommit: rf.commitIndex,
				PrevLogIndex: nextIndex - 1,
				PrevLogTerm:  prevLogEntry.Term,
				Entries:      rf.log.getLogEntriesStartedFrom(nextIndex),
			}
		}
	}

	rf.mu.Unlock()

	for i := range peersCount {
		if i == me {
			continue
		}
		go func() {
			if allAppendEntriesArgs[i] != nil {
				args := allAppendEntriesArgs[i]
				reply := AppendEntriesReply{}
				ok := rf.sendAppendEntries(i, args, &reply)

				if ok && reply.Term == term {
					rf.mu.Lock()
					if reply.Success {
						matchIndex := args.PrevLogIndex + len(args.Entries)
						rf.nextIndex[i] = matchIndex + 1
						rf.matchIndex[i] = matchIndex

						// leader can commit a log entry if it has been
						// replicated in the majority servers only if the
						// log entry is in the current term as explained
						// in paper's figure 8
						if len(args.Entries) > 0 && args.Entries[len(args.Entries)-1].Term == rf.currentTerm {
							rf.commitIfPossible()
						}
					} else {
						rf.nextIndex[i] = max(reply.XIndex, 1)
					}
					rf.Debug(fmt.Sprintf("sendAppendEntries(%d) done", i))
					rf.mu.Unlock()
				}
			} else {
				args := allInstallSnapshotArgs[i]
				reply := InstallSnapshotReply{}
				ok := rf.sendInstallSnapshot(i, args, &reply)

				if ok && reply.Success {
					rf.mu.Lock()
					rf.nextIndex[i] = args.SnapshotIndex + 1
					rf.matchIndex[i] = args.SnapshotIndex
					rf.commitIfPossible()
					rf.Debug(fmt.Sprintf("sendInstallSnapshot(%d) done", i))
					rf.mu.Unlock()
				}
			}
		}()
	}
}

func (rf *Raft) applyLogEntriesIfNeeded() {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.lastApplied < rf.commitIndex {
		rf.Debug("applyLogEntries start")
		for i := rf.lastApplied + 1; i <= rf.commitIndex; i++ {
			logEntry, ok := rf.log.getLogEntry(i)
			if !ok {
				log.Fatalln("unimplemented!")
			}

			var msg raftapi.ApplyMsg
			msg.CommandValid = true
			msg.Command = logEntry.Command
			msg.CommandIndex = i
			rf.applyCh <- msg
		}
		rf.lastApplied = rf.commitIndex
		rf.Debug("applyLogEntries done")
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
	rf.applyCh = applyCh

	// Your initialization code here (3A, 3B, 3C).
	rf.state = Follower
	rf.lastHeartbeatAt = time.Now()
	rf.currentTerm = 0
	rf.voteFor = NoVote
	rf.log = newRaftLog()
	rf.commitIndex = 0
	rf.lastApplied = 0
	rf.nextIndex = make([]int, len(peers))
	rf.matchIndex = make([]int, len(peers))

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	rf.Debug("make a server")

	go func() {
		for !rf.killed() {
			rf.startElectionIfNeeded()
			// pause for a random amount of time between 50 and 350
			// milliseconds.
			ms := 50 + (rand.Int63() % 300)
			time.Sleep(time.Duration(ms) * time.Millisecond)
		}
	}()

	go func() {
		for !rf.killed() {
			rf.sendAppendEntriesIfNeeded()
			time.Sleep(sendAppendEntriesInterval)
		}
	}()

	go func() {
		for !rf.killed() {
			rf.applyLogEntriesIfNeeded()
			time.Sleep(applyCommittedEntriesInterval)
		}
	}()

	return rf
}

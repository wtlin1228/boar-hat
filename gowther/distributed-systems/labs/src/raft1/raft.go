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

type ServerState int

const (
	Leader ServerState = iota
	Follower
	Candidate
)

func (s ServerState) String() string {
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

const NoVote = -1

const electionTimeout time.Duration = 500 * time.Millisecond

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

	serverState     ServerState // Follower | Candidate | Leader
	lastHeartbeatAt time.Time   // to be used with election timeout
	currentTerm     int         // latest term server has see, increase monotonically

	// candidate id that received vote in the current term,
	// -1 means hasn't voted for current term
	voteFor int

	// log entries, each entry contains command for state machine,
	// and term when entry was received by leader
	log []LogEntry

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
}

// caller is responsible for holding the lock
func (rf *Raft) getLastLogEntryTerm() int {
	return rf.log[rf.getLastLogEntryIndex()].Term
}

// caller is responsible for holding the lock
func (rf *Raft) getLastLogEntryIndex() int {
	return len(rf.log) - 1
}

// caller is responsible for holding the lock
func (rf *Raft) setServerState(state ServerState) {
	DPrintf("[%d] %-9s - change state to %s", rf.me, rf.serverState, state)
	rf.serverState = state
}

// caller is responsible for holding the lock
// the current term might be changed with or without voting for some server
func (rf *Raft) setCurrentTerm(term int, voteFor int) {
	DPrintf("[%d] %-9s - increase current term to %d", rf.me, rf.serverState, term)
	rf.currentTerm = term
	rf.setVoteFor(voteFor)
	rf.persist()
}

// caller is responsible for holding the lock
func (rf *Raft) setLastHeartbeatAt(at time.Time) {
	DPrintf("[%d] %-9s - update last heartbeat at to %s", rf.me, rf.serverState, at.Format("15:04:05.000"))
	rf.lastHeartbeatAt = at
}

// caller is responsible for holding the lock
func (rf *Raft) setVoteFor(id int) {
	if id != NoVote {
		DPrintf("[%d] %-9s - vote for %d", rf.me, rf.serverState, id)
	}
	rf.voteFor = id
	rf.persist()
}

// caller is responsible for holding the lock
func (rf *Raft) createAndAppendLogEntry(command any) {
	DPrintf("[%d] %-9s - create and append log entry, logIndex=%d, currentTerm=%d, command=%v", rf.me, rf.serverState, rf.getLastLogEntryIndex()+1, rf.currentTerm, command)
	rf.log = append(rf.log, LogEntry{Term: rf.currentTerm, Command: command})
	DPrintf("[%d] %-9s - log=%v", rf.me, rf.serverState, rf.log)
	rf.persist()
}

// caller is responsible for holding the lock
func (rf *Raft) replaceLogEntries(startFrom int, entries []LogEntry) {
	// if rf.commitIndex > startFrom {
	// 	log.Fatalf("[%d] can't replace already committed log, commitIndex=%d, startFrom=%d, entryCount=%d", rf.me, rf.commitIndex, startFrom, len(entries))
	// }

	if len(entries) > 0 {
		DPrintf("[%d] %-9s - replace log entries, startFrom=%d, entryCount=%d", rf.me, rf.serverState, startFrom, len(entries))
		rf.log = slices.Delete(rf.log, startFrom, len(rf.log))
		rf.log = slices.Concat(rf.log, entries)
		DPrintf("[%d] %-9s - log=%v", rf.me, rf.serverState, rf.log)
		rf.persist()
	}
}

// caller is responsible for holding the lock
// should only and must be called when selected as leader
func (rf *Raft) initNextIndex() {
	nextIndex := rf.getLastLogEntryIndex() + 1
	DPrintf("[%d] %-9s - init []nextIndex to %d", rf.me, rf.serverState, nextIndex)
	for i := range rf.nextIndex {
		rf.nextIndex[i] = nextIndex
	}
}

// caller is responsible for holding the lock
// should only and must be called when selected as leader
func (rf *Raft) initMatchIndex() {
	DPrintf("[%d] %-9s - init []matchIndex to 0", rf.me, rf.serverState)
	for i := range rf.matchIndex {
		rf.matchIndex[i] = 0
	}
}

// caller is responsible for holding the lock
func (rf *Raft) setNextIndex(server int, i int) {
	if rf.nextIndex[server] != i {
		DPrintf("[%d] %-9s - update nextIndex[%d] from %d to %d", rf.me, rf.serverState, server, rf.nextIndex[server], i)
		rf.nextIndex[server] = i
	}
}

// caller is responsible for holding the lock
func (rf *Raft) setMatchIndex(server int, i int) {
	if i < rf.commitIndex {
		log.Fatalf("matchIndex must only increase monotonically, matchIndex[%d] from %d to %d", server, rf.matchIndex[server], i)
		return
	}

	if i > rf.matchIndex[server] {
		DPrintf("[%d] %-9s - increase matchIndex[%d] from %d to %d", rf.me, rf.serverState, server, rf.matchIndex[server], i)
		rf.matchIndex[server] = i
	}
}

// caller is responsible for holding the lock
func (rf *Raft) setCommitIndex(i int) {
	if i < rf.commitIndex {
		// log.Fatalf("[%d] commitIndex must only increase monotonically, commitIndex from %d to %d", rf.me, rf.commitIndex, i)

		// when a server restarts and wins the election again, the commit index will be reset to 0
		// but the commit index of living servers will be larger than 0
		// in this case, we don't decrease the living servers' commit index
		return
	}

	if i > rf.getLastLogEntryIndex() {
		log.Fatalf("[%d] commitIndex can't exceed the log, commitIndex from %d to %d, lastLogIndex=%d", rf.me, rf.commitIndex, i, rf.getLastLogEntryIndex())
		return
	}

	if i > rf.commitIndex {
		DPrintf("[%d] %-9s - increase commitIndex from %d to %d", rf.me, rf.serverState, rf.commitIndex, i)
		rf.commitIndex = i
	}
}

// caller is responsible for holding the lock
func (rf *Raft) setLastApplied(i int) {
	if i < rf.lastApplied {
		log.Fatalf("lastApplied must only increase monotonically, lastApplied from %d to %d", rf.lastApplied, i)
		return
	}

	if i > rf.lastApplied {
		DPrintf("[%d] %-9s - increase lastApplied from %d to %d", rf.me, rf.serverState, rf.lastApplied, i)
		rf.lastApplied = i
	}
}

// caller is responsible for holding the lock
func (rf *Raft) getXTermAndXIndex(term int, index int) (int, int) {
	xIndex := min(index, rf.getLastLogEntryIndex())

	// 1. find the largest term T that is smaller than given term
	for xIndex > 1 && rf.log[xIndex].Term >= term {
		xIndex -= 1
	}
	xTerm := rf.log[xIndex].Term

	// 2. find the first log for term T
	for xIndex > 1 && rf.log[xIndex-1].Term == xTerm {
		xIndex -= 1
	}

	return xTerm, xIndex
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	// Your code here (3A).
	rf.mu.Lock()
	defer rf.mu.Unlock()
	return rf.currentTerm, rf.serverState == Leader
}

// save Raft's persistent state to stable storage,
// where it can later be retrieved after a crash and restart.
// see paper's Figure 2 for a description of what should be persistent.
// before you've implemented snapshots, you should pass nil as the
// second argument to persister.Save().
// after you've implemented snapshots, pass the current snapshot
// (or nil if there's not yet a snapshot).
func (rf *Raft) persist() {
	DPrintf("[%d] %-9s - persist data, currentTerm=%d, voteFor=%d, log=%v", rf.me, rf.serverState, rf.currentTerm, rf.voteFor, rf.log)
	// Your code here (3C).
	// Example:
	// w := new(bytes.Buffer)
	// e := labgob.NewEncoder(w)
	// e.Encode(rf.xxx)
	// e.Encode(rf.yyy)
	// raftstate := w.Bytes()
	// rf.persister.Save(raftstate, nil)
	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	e.Encode(rf.currentTerm)
	e.Encode(rf.voteFor)
	e.Encode(rf.log)
	raftstate := w.Bytes()
	rf.persister.Save(raftstate, nil)
}

// restore previously persisted state.
func (rf *Raft) readPersist(data []byte) {
	if len(data) < 1 { // bootstrap without any state?
		return
	}
	// Your code here (3C).
	// Example:
	// r := bytes.NewBuffer(data)
	// d := labgob.NewDecoder(r)
	// var xxx
	// var yyy
	// if d.Decode(&xxx) != nil ||
	//    d.Decode(&yyy) != nil {
	//   error...
	// } else {
	//   rf.xxx = xxx
	//   rf.yyy = yyy
	// }
	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	var currentTerm int
	var voteFor int
	var logEntries []LogEntry
	if d.Decode(&currentTerm) != nil ||
		d.Decode(&voteFor) != nil ||
		d.Decode(&logEntries) != nil {
		log.Fatalln("fail to decode the persist data")
	} else {
		rf.currentTerm = currentTerm
		rf.voteFor = voteFor
		rf.log = logEntries
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
	// Your code here (3D).

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
	// Your code here (3A, 3B).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	DPrintf("[%d] %-9s - receive RequestVote RPC from [%d] on term %d", rf.me, rf.serverState, args.CandidateId, args.Term)

	if rf.currentTerm >= args.Term {
		reply.Term = rf.currentTerm
		reply.VoteGranted = false
		return
	}

	rf.setCurrentTerm(args.Term, NoVote)
	if rf.serverState != Follower {
		rf.setServerState(Follower)
	}
	reply.Term = args.Term

	// Only vote for the candidate if its log is more update-to-date
	// RULE:
	//   If the logs have last entries with different terms,
	//   then the log with the later term is more up-to-date.
	//   If the logs end with the same term, then whichever log is longer
	//   is more up-to-date.
	lastLogEntryTerm := rf.getLastLogEntryTerm()
	lastLogEntryIndex := rf.getLastLogEntryIndex()
	if args.LastLogTerm < lastLogEntryTerm || (args.LastLogTerm == lastLogEntryTerm && args.LastLogIndex < lastLogEntryIndex) {
		reply.VoteGranted = false
	} else {
		rf.setVoteFor(args.CandidateId)
		reply.VoteGranted = true
	}
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
	DPrintf("[%d] %-9s - send RequestVote RPC to [%d] on term %d", args.CandidateId, Candidate, server, args.Term)

	ok := rf.peers[server].Call("Raft.RequestVote", args, reply)

	rf.mu.Lock()
	defer rf.mu.Unlock()

	if reply.Term == 0 {
		DPrintf("[%d] %-9s - doesn't get the vote from [%d], timeout", args.CandidateId, rf.serverState, server)
		// timeout, do nothing
	} else if rf.currentTerm < reply.Term {
		DPrintf("[%d] %-9s - doesn't get the vote from [%d], currentTerm %d is smaller than other's term %d", args.CandidateId, rf.serverState, server, rf.currentTerm, reply.Term)

		rf.setCurrentTerm(reply.Term, NoVote)
		if rf.serverState != Follower {
			rf.setServerState(Follower)
		}
	} else if reply.VoteGranted == true {
		DPrintf("[%d] %-9s - receive vote from [%d], term is %d", args.CandidateId, rf.serverState, server, reply.Term)
	} else if reply.VoteGranted == false {
		DPrintf("[%d] %-9s - doesn't get the vote from [%d], term is %d", args.CandidateId, rf.serverState, server, reply.Term)
	}

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

func (args AppendEntriesArgs) String() string {
	return fmt.Sprintf(
		"{\n  Term: %d,\n  LeaderId: %d,\n  LeaderCommit: %d,\n  PrevLogIndex: %d,\n  PrevLogTerm: %d,\n  Entries=%v\n}",
		args.Term, args.LeaderId, args.LeaderCommit, args.PrevLogIndex, args.PrevLogTerm, args.Entries,
	)
}

type AppendEntriesReply struct {
	Term    int  // currentTerm, for leader to update itself
	Success bool // true if follower contained entry matching prevLogIndex and prevLogTerm

	XTerm  int // faster backup: term of conflicting entry
	XIndex int // faster backup: index of the first entry within XTerm
	XLen   int // faster backup: length of the log
}

func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	DPrintf("[%d] %-9s - receive AppendEntries RPC from [%d] on term %d", rf.me, rf.serverState, args.LeaderId, args.Term)

	if rf.currentTerm > args.Term {
		reply.Term = rf.currentTerm
		return
	}

	rf.setLastHeartbeatAt(time.Now())
	if rf.serverState != Follower {
		rf.setServerState(Follower)
	}
	if rf.currentTerm < args.Term {
		rf.setCurrentTerm(args.Term, NoVote)
	}

	reply.Term = args.Term

	if rf.getLastLogEntryIndex() >= args.PrevLogIndex && rf.log[args.PrevLogIndex].Term == args.PrevLogTerm {
		// commit index and log entries need to be updated together, or the server
		// will apply the wrong command
		rf.replaceLogEntries(args.PrevLogIndex+1, args.Entries)
		rf.setCommitIndex(args.LeaderCommit)
		reply.Success = true
		return
	}

	reply.Success = false

	// for faster backup
	xTerm, xIndex := rf.getXTermAndXIndex(args.PrevLogTerm, args.PrevLogIndex)
	reply.XTerm = xTerm
	reply.XIndex = xIndex
	reply.XLen = len(rf.log)
}

func (rf *Raft) sendAppendEntries(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	DPrintf("[%d] %-9s - send AppendEntries RPC to [%d] on term %d\n%s", args.LeaderId, Leader, server, args.Term, args)

	ok := rf.peers[server].Call("Raft.AppendEntries", args, reply)

	rf.mu.Lock()
	defer rf.mu.Unlock()

	if reply.Term == 0 {
		DPrintf("[%d] %-9s - failed to replicate %d log entries to [%d], timeout", rf.me, rf.serverState, len(args.Entries), server)

		// timeout, do nothing
	} else if rf.currentTerm < reply.Term {
		DPrintf("[%d] %-9s - failed to replicate %d log entries to [%d], currentTerm %d is smaller than other's term %d", rf.me, rf.serverState, len(args.Entries), server, rf.currentTerm, reply.Term)

		rf.setCurrentTerm(reply.Term, NoVote)
		if rf.serverState != Follower {
			rf.setServerState(Follower)
		}
	} else if reply.Success == false {
		DPrintf("[%d] %-9s - failed to replicate %d log entries to [%d], previous log(term=%d,index=%d) mismatch", rf.me, rf.serverState, len(args.Entries), server, args.PrevLogTerm, args.PrevLogIndex)

		// slower backup
		// if rf.nextIndex[server] > 1 {
		// 	rf.setNextIndex(server, rf.nextIndex[server]-1)
		// }

		// faster backup
		if reply.XIndex >= 1 {
			rf.setNextIndex(server, reply.XIndex)
		}
	} else if reply.Success == true {
		DPrintf("[%d] %-9s - successfully replicate %d log entries to [%d], term is %d", args.LeaderId, rf.serverState, len(args.Entries), server, reply.Term)

		nextIndex := rf.nextIndex[server] + len(args.Entries)
		rf.setNextIndex(server, nextIndex)
		rf.setMatchIndex(server, nextIndex-1)

		// leader can commit a log entry if it has been replicated in the
		// majority servers only if the log entry is in the current term
		// as explained in paper's figure 8
		if len(args.Entries) > 0 && args.Entries[len(args.Entries)-1].Term == rf.currentTerm {
			matchIndexes := slices.Clone(rf.matchIndex)
			slices.Sort(matchIndexes)
			l := len(matchIndexes)
			commitIndex := matchIndexes[l-l/2]
			// when the leader is just selected, matchIndexes will be reset to 0
			// and the commitIndex shouldn't be decreased
			if commitIndex > rf.commitIndex {
				rf.setCommitIndex(commitIndex)
			}
		}
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

	index := rf.getLastLogEntryIndex() + 1
	term := rf.currentTerm
	isLeader := rf.serverState == Leader

	if isLeader {
		rf.createAndAppendLogEntry(command)
	}

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
	rf.mu.Lock()
	defer rf.mu.Unlock()

	atomic.StoreInt32(&rf.dead, 1)
	// Your code here, if desired.
	DPrintf("[%d] %-9s - killed on term %d\n", rf.me, rf.serverState, rf.currentTerm)
}

func (rf *Raft) killed() bool {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

func (rf *Raft) ticker() {
	for !rf.killed() {
		// Your code here (3A)
		// Check if a leader election should be started.

		rf.mu.Lock()
		// handle election timeout
		if rf.serverState == Follower && rf.lastHeartbeatAt.Add(electionTimeout).Before(time.Now()) {
			rf.setServerState(Candidate)
		}
		shouldStartLeaderElection := rf.serverState == Candidate
		rf.mu.Unlock()

		if shouldStartLeaderElection {
			rf.mu.Lock()

			rf.setCurrentTerm(rf.currentTerm+1, rf.me)

			me := rf.me
			thisTerm := rf.currentTerm
			peersCount := len(rf.peers)
			lastLogIndex := rf.getLastLogEntryIndex()
			lastLogTerm := rf.getLastLogEntryTerm()

			rf.mu.Unlock()

			// old elections shouldn't block the new elections
			go rf.startNewElection(me, thisTerm, peersCount, lastLogIndex, lastLogTerm)
		}

		// pause for a random amount of time between 50 and 350
		// milliseconds.
		ms := 50 + (rand.Int63() % 300)
		time.Sleep(time.Duration(ms) * time.Millisecond)
	}
}

func (rf *Raft) startNewElection(me int, thisTerm int, peersCount int, lastLogIndex int, lastLogTerm int) {
	c := make(chan bool)

	for i := range peersCount {
		if i != me {
			go func() {
				args := RequestVoteArgs{
					Term:         thisTerm,
					CandidateId:  me,
					LastLogIndex: lastLogIndex,
					LastLogTerm:  lastLogTerm,
				}
				reply := RequestVoteReply{}
				ok := rf.sendRequestVote(i, &args, &reply)
				if ok && reply.Term == thisTerm {
					c <- reply.VoteGranted
				} else {
					c <- false
				}
			}()
		}
	}

	count := 1 // because it votes for itself
	finished := 0

	for {
		vote := <-c
		if vote {
			count += 1
		}
		finished += 1

		rf.mu.Lock()
		serverState := rf.serverState
		rf.mu.Unlock()

		if serverState == Follower || serverState == Leader {
			break
		}

		if count > peersCount/2 {
			// Only the candidate is allowed to be promoted to leader, not follower.
			// It could be follower because another server wins the election.
			if serverState == Candidate {
				rf.mu.Lock()
				rf.setServerState(Leader)
				rf.initNextIndex()
				rf.initMatchIndex()
				rf.mu.Unlock()

				rf.sendHeartbeats()
			}
			break
		}

		if finished == peersCount {
			// stay as candidate, do the leader election and request vote again
			break
		}
	}
}

// send heartbeats if it's the leader
func (rf *Raft) sendHeartbeats() {
	rf.mu.Lock()

	if rf.serverState != Leader {
		rf.mu.Unlock()
		return
	}

	me := rf.me

	allArgs := make([]AppendEntriesArgs, len(rf.peers))
	for i := range len(rf.peers) {
		if i == me {
			continue
		}
		nextIndex := rf.nextIndex[i]
		allArgs[i] = AppendEntriesArgs{
			Term:         rf.currentTerm,
			LeaderId:     rf.me,
			LeaderCommit: rf.commitIndex,
			PrevLogIndex: nextIndex - 1,
			PrevLogTerm:  rf.log[nextIndex-1].Term,
			Entries:      rf.log[nextIndex:],
		}
	}

	rf.mu.Unlock()

	for i, args := range allArgs {
		if i == me {
			continue
		}
		reply := AppendEntriesReply{}
		go rf.sendAppendEntries(i, &args, &reply)
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

	// Your initialization code here (3A, 3B, 3C).
	rf.serverState = Follower
	rf.lastHeartbeatAt = time.Now()
	rf.currentTerm = 0
	rf.voteFor = NoVote
	// Raft log is 1-indexed, but we suggest that you view it as 0-indexed,
	// and starting out with an entry (at index=0) that has term 0.
	// That allows the very first AppendEntries RPC to contain 0 as PrevLogIndex,
	// and be a valid index into the log.
	rf.log = append(rf.log, LogEntry{Term: rf.currentTerm, Command: nil})
	rf.commitIndex = 0
	rf.lastApplied = 0
	rf.nextIndex = make([]int, len(peers))
	rf.matchIndex = make([]int, len(peers))

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	DPrintf("Make a raft server, me=%d, currentTerm=%d, voteFor=%d, log=%v", rf.me, rf.currentTerm, rf.voteFor, rf.log)

	// start ticker goroutine to start elections
	go rf.ticker()

	go func() {
		for !rf.killed() {
			rf.sendHeartbeats()
			// The tester requires that the leader send heartbeat RPCs no more than ten times
			// per second.
			time.Sleep(200 * time.Millisecond)
		}
	}()

	go func() {
		for !rf.killed() {
			rf.mu.Lock()
			for i := rf.lastApplied + 1; i <= rf.commitIndex; i++ {
				DPrintf(
					"[%d] %-9s - apply log\n{\n  logIndex=%d,\n  logTerm=%d,\n  logCommand=%v,\n}",
					rf.me, rf.serverState, i, rf.log[i].Term, rf.log[i].Command,
				)
				var msg raftapi.ApplyMsg
				msg.CommandValid = true
				msg.Command = rf.log[i].Command
				msg.CommandIndex = i
				applyCh <- msg

				rf.setLastApplied(i)
			}
			rf.mu.Unlock()

			time.Sleep(10 * time.Millisecond)
		}
	}()

	return rf
}

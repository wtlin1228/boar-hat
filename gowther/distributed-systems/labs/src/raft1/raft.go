package raft

// The file raftapi/raft.go defines the interface that raft must
// expose to servers (or the tester), but see comments below for each
// of these functions for more details.
//
// Make() creates a new raft peer that implements the raft interface.

import (
	"fmt"
	"log"
	"math/rand"
	"sync"
	"sync/atomic"
	"time"

	"6.5840/labrpc"
	"6.5840/raftapi"
	tester "6.5840/tester1"
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

type Snapshot struct {
	LastIncludedIndex int
	LastIncludedTerm  int
	StateMachineState []byte
}

func (s *Snapshot) String() string {
	if s == nil {
		return ""
	}
	return fmt.Sprintf("Snapshot{LastIncludedIndex=%d, LastIncludedTerm=%d}", s.LastIncludedIndex, s.LastIncludedTerm)
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
	currentTerm int
	votedFor    int
	log         []LogEntry
	logOffset   int
	snapshot    *Snapshot

	// Volatile state
	state               State
	commitIndex         int
	lastApplied         int
	nextIndex           []int
	matchIndex          []int
	lastAppendEntriesAt time.Time  // for throttling the AppendEntries RPC calls
	lastHeartbeatAt     time.Time  // updated when an Heartbeat RPC is received
	applyChMu           sync.Mutex // for preventing sending on closed channel
	applyCh             chan raftapi.ApplyMsg
}

func (rf *Raft) debug(format string, a ...interface{}) {
	DPrintf("[Raft_%d] term:%d %-9s  - %s", rf.me, rf.currentTerm, rf.state, fmt.Sprintf(format, a...))
}

func (rf *Raft) getLogEntry(logIndex int) *LogEntry {
	if logIndex == 0 {
		return &rf.log[0]
	}

	if logIndex < rf.logOffset {
		log.Fatalf("log has been trimmed, logIndex=%d, rf.logOffset=%d\n", logIndex, rf.logOffset)
	}

	if logIndex > rf.logOffset+len(rf.log)-1 {
		log.Fatalf("log doesn't exist, logIndex=%d, log count=%d\n", logIndex, rf.logOffset+len(rf.log)-1)
	}

	return &rf.log[logIndex-rf.logOffset]
}

func (rf *Raft) catchUpTerm(term int) {
	if rf.currentTerm >= term {
		log.Fatalf("no need to catch up term, term=%d\n", term)
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
		log.Fatalf("no need to change state, state=%s\n", state)
	}
	rf.debug("change state, %s -> %s", rf.state, state)
	rf.state = state
}

func (rf *Raft) increaseCommitIndex(index int) {
	if index <= rf.commitIndex {
		log.Fatalf("commit index can only increase monotonically, index=%d, rf.commitIndex=%d\n", index, rf.commitIndex)
	}
	rf.debug("increase commit index, %d -> %d", rf.commitIndex, index)
	rf.commitIndex = index
}

func (rf *Raft) increaseLastApplied(index int) {
	if index <= rf.lastApplied {
		log.Fatalf("last applied can only increase monotonically, index=%d, rf.lastApplied=%d\n", index, rf.lastApplied)
	}
	rf.debug("increase last applied, %d -> %d", rf.lastApplied, index)
	rf.lastApplied = index
}

func (rf *Raft) updateSnapshot(snapshot *Snapshot) {
	if rf.snapshot != nil && snapshot.LastIncludedIndex <= rf.snapshot.LastIncludedIndex {
		log.Fatalf("no need to update snapshot, snapshot.LastIncludedIndex=%d, rf.snapshot.LastIncludedIndex=%d\n", snapshot.LastIncludedIndex, rf.snapshot.LastIncludedIndex)
	}
	rf.debug("update snapshot, %s -> %s", rf.snapshot, snapshot)
	rf.snapshot = snapshot
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
	// w := new(bytes.Buffer)
	// e := labgob.NewEncoder(w)
	// e.Encode(rf.currentTerm)
	// e.Encode(rf.votedFor)
	// e.Encode(rf.log.data)
	// e.Encode(rf.log.startAt)
	// raftstate := w.Bytes()
	// if len(rf.snapshot) > 0 {
	// 	rf.persister.Save(raftstate, rf.snapshot)
	// } else {
	// 	rf.persister.Save(raftstate, nil)
	// }
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

type HeartbeatArgs struct {
	Term     int // leader's term
	LeaderId int // so follower can redirect clients
}

type HeartbeatReply struct {
	Term int // currentTerm, for leader to update itself
}

func (rf *Raft) Heartbeat(args *HeartbeatArgs, reply *HeartbeatReply) {

}

func (rf *Raft) sendHeartbeat(server int, args *HeartbeatArgs, reply *HeartbeatReply) bool {
	ok := rf.peers[server].Call("Raft.Heartbeat", args, reply)
	return ok
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
	ok := rf.peers[server].Call("Raft.RequestVote", args, reply)
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

}

func (rf *Raft) sendAppendEntries(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	ok := rf.peers[server].Call("Raft.AppendEntries", args, reply)
	return ok
}

type InstallSnapshotArgs struct {
	Term              int    // leader's term
	LeaderId          int    // so follower can redirect clients
	lastIncludedIndex int    // the snapshot replaces all entries up through and including this index
	LastIncludedTerm  int    // term of lastIncludedIndex
	Offset            int    // byte offset where chunk is positioned in the snapshot file
	Data              []byte // raw bytes of the snapshot chunk, starting at offset
	Done              bool   // true if this is the last chunk
}

type InstallSnapshotReply struct {
	Term int // currentTerm, for leader to update itself
}

func (rf *Raft) InstallSnapshot(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {

}

func (rf *Raft) sendInstallSnapshot(server int, args *InstallSnapshotArgs, reply *InstallSnapshotReply) bool {
	ok := rf.peers[server].Call("Raft.InstallSnapshot", args, reply)
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
	index := -1
	term := -1
	isLeader := true

	// Your code here (3B).

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

func (rf *Raft) ticker() {
	for rf.killed() == false {

		// Your code here (3A)
		// Check if a leader election should be started.

		// pause for a random amount of time between 50 and 350
		// milliseconds.
		ms := 50 + (rand.Int63() % 300)
		time.Sleep(time.Duration(ms) * time.Millisecond)
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

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	// start ticker goroutine to start elections
	go rf.ticker()

	return rf
}

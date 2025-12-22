package raft

// The file raftapi/raft.go defines the interface that raft must
// expose to servers (or the tester), but see comments below for each
// of these functions for more details.
//
// Make() creates a new raft peer that implements the raft interface.

import (
	//	"bytes"

	"math/rand"
	"sync"
	"sync/atomic"
	"time"

	//	"6.5840/labgob"
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
	// Your code here (3C).
	// Example:
	// w := new(bytes.Buffer)
	// e := labgob.NewEncoder(w)
	// e.Encode(rf.xxx)
	// e.Encode(rf.yyy)
	// raftstate := w.Bytes()
	// rf.persister.Save(raftstate, nil)
}

// restore previously persisted state.
func (rf *Raft) readPersist(data []byte) {
	if data == nil || len(data) < 1 { // bootstrap without any state?
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

// example RequestVote RPC arguments structure.
// field names must start with capital letters!
type RequestVoteArgs struct {
	// Your data here (3A, 3B).
	Term        int // candidate's term
	CandidateId int // candidate requesting vote
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

	rf.setCurrentTerm(args.Term, args.CandidateId)
	if rf.serverState != Follower {
		rf.setServerState(Follower)
	}

	reply.Term = args.Term
	reply.VoteGranted = true
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

	if ok {
		DPrintf("[%d] %-9s - receive RequestVote RPC response from [%d], term is %d, vote is %v", args.CandidateId, rf.serverState, server, reply.Term, reply.VoteGranted)
	} else {
		DPrintf("[%d] %-9s - receive RequestVote RPC response from [%d], timeout", args.CandidateId, rf.serverState, server)
	}

	if rf.currentTerm < reply.Term {
		rf.setCurrentTerm(reply.Term, NoVote)
		if rf.serverState != Follower {
			rf.setServerState(Follower)
		}
	}

	return ok
}

type AppendEntriesArgs struct {
	Term     int // leader's term
	LeaderId int // so follower can redirect clients
}

type AppendEntriesReply struct {
	Term    int  // currentTerm, for leader to update itself
	Success bool // true if follower contained entry matching prevLogIndex and prevLogTerm
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
}

func (rf *Raft) sendAppendEntries(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	DPrintf("[%d] %-9s - send AppendEntries RPC to [%d] on term %d", args.LeaderId, Leader, server, args.Term)

	ok := rf.peers[server].Call("Raft.AppendEntries", args, reply)

	rf.mu.Lock()
	defer rf.mu.Unlock()

	DPrintf("[%d] %-9s - receive AppendEntries RPC response from [%d], term is %d", args.LeaderId, rf.serverState, server, reply.Term)

	if rf.currentTerm < reply.Term {
		rf.setCurrentTerm(reply.Term, NoVote)
		if rf.serverState != Follower {
			rf.setServerState(Follower)
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
	rf.mu.Lock()
	defer rf.mu.Unlock()

	atomic.StoreInt32(&rf.dead, 1)
	// Your code here, if desired.
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

			rf.mu.Unlock()

			// old elections shouldn't block the new elections
			go rf.startNewElection(me, thisTerm, peersCount)
		}

		// pause for a random amount of time between 50 and 350
		// milliseconds.
		ms := 50 + (rand.Int63() % 300)
		time.Sleep(time.Duration(ms) * time.Millisecond)
	}
}

func (rf *Raft) startNewElection(me int, thisTerm int, peersCount int) {
	c := make(chan bool)

	for i := range peersCount {
		if i != me {
			go func() {
				args := RequestVoteArgs{Term: thisTerm, CandidateId: me}
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

	isLeader := rf.serverState == Leader
	peersCount := len(rf.peers)
	thisTerm := rf.currentTerm
	me := rf.me

	rf.mu.Unlock()

	if isLeader {
		for i := range peersCount {
			if i != me {
				args := AppendEntriesArgs{thisTerm, me}
				reply := AppendEntriesReply{}
				go rf.sendAppendEntries(i, &args, &reply)
			}
		}
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

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

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

	return rf
}

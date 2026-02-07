package rsm

import (
	"fmt"
	"log"
	"slices"
	"sync"

	"6.5840/kvsrv1/rpc"
	"6.5840/labrpc"
	raft "6.5840/raft1"
	"6.5840/raftapi"
	tester "6.5840/tester1"

	"github.com/google/uuid"
)

const Debug = true

var useRaftStateMachine bool // to plug in another raft besided raft1

type Op struct {
	// Your definitions here.
	// Field names must start with capital letters,
	// otherwise RPC will break.
	Me  int
	Id  string
	Req any
}

type OpResult struct {
	val any
	ok  bool
}

type OpQueueEntry struct {
	logTerm  int
	logIndex int
	ch       chan OpResult
	op       *Op
}

// A server (i.e., ../server.go) that wants to replicate itself calls
// MakeRSM and must implement the StateMachine interface.  This
// interface allows the rsm package to interact with the server for
// server-specific operations: the server must implement DoOp to
// execute an operation (e.g., a Get or Put request), and
// Snapshot/Restore to snapshot and restore the server's state.
type StateMachine interface {
	DoOp(any) any
	Snapshot() []byte
	Restore([]byte)
}

type RSM struct {
	mu           sync.Mutex
	me           int
	rf           raftapi.Raft
	applyCh      chan raftapi.ApplyMsg
	maxraftstate int // snapshot if log grows this big
	sm           StateMachine
	// Your definitions here.

	opQueue []OpQueueEntry
}

func (rf *RSM) Debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[%d] - %s\n", rf.me, fmt.Sprintf(format, a...))
	}
}

// servers[] contains the ports of the set of
// servers that will cooperate via Raft to
// form the fault-tolerant key/value service.
//
// me is the index of the current server in servers[].
//
// the k/v server should store snapshots through the underlying Raft
// implementation, which should call persister.SaveStateAndSnapshot() to
// atomically save the Raft state along with the snapshot.
// The RSM should snapshot when Raft's saved state exceeds maxraftstate bytes,
// in order to allow Raft to garbage-collect its log. if maxraftstate is -1,
// you don't need to snapshot.
//
// MakeRSM() must return quickly, so it should start goroutines for
// any long-running work.
func MakeRSM(servers []*labrpc.ClientEnd, me int, persister *tester.Persister, maxraftstate int, sm StateMachine) *RSM {
	rsm := &RSM{
		me:           me,
		maxraftstate: maxraftstate,
		applyCh:      make(chan raftapi.ApplyMsg),
		sm:           sm,
	}
	if !useRaftStateMachine {
		rsm.rf = raft.Make(servers, me, persister, rsm.applyCh)
	}

	go func() {
		for cmd := range rsm.applyCh {
			rsm.Debug("receive from applyCh, %v", cmd)
			op := cmd.Command.(Op)

			rsm.mu.Lock()
			for len(rsm.opQueue) > 0 {
				if rsm.opQueue[0].logIndex >= cmd.CommandIndex {
					break
				}
				rsm.opQueue[0].ch <- OpResult{
					val: nil,
					ok:  false,
				}
				rsm.Debug("remove entry %v", rsm.opQueue[0])
				rsm.opQueue = slices.Delete(rsm.opQueue, 0, 1)
			}
			rsm.mu.Unlock()

			res := rsm.sm.DoOp(op.Req)

			rsm.mu.Lock()
			if len(rsm.opQueue) > 0 && rsm.opQueue[0].op.Id == op.Id {
				rsm.opQueue[0].ch <- OpResult{
					val: res,
					ok:  true,
				}
				rsm.Debug("remove entry %v", rsm.opQueue[0])
				rsm.opQueue = slices.Delete(rsm.opQueue, 0, 1)
			}
			rsm.mu.Unlock()
		}
	}()

	return rsm
}

func (rsm *RSM) Raft() raftapi.Raft {
	return rsm.rf
}

// Submit a command to Raft, and wait for it to be committed.  It
// should return ErrWrongLeader if client should find new leader and
// try again.
func (rsm *RSM) Submit(req any) (rpc.Err, any) {

	// Submit creates an Op structure to run a command through Raft;
	// for example: op := Op{Me: rsm.me, Id: id, Req: req}, where req
	// is the argument to Submit and id is a unique id for the op.

	op := Op{
		Me:  rsm.me,
		Id:  uuid.NewString(),
		Req: req,
	}

	rsm.Debug("Raft.Start        - Op=%v", op)

	index, term, isLeader := rsm.Raft().Start(op)

	rsm.Debug("Raft.Start return - Op=%v, index=%d, term=%d, isLeader=%v", op, index, term, isLeader)

	if !isLeader {
		return rpc.ErrWrongLeader, nil // i'm dead, try another server.
	}

	newEntry := OpQueueEntry{
		logTerm:  term,
		logIndex: index,
		ch:       make(chan OpResult, 1),
		op:       &op,
	}

	rsm.mu.Lock()
	// Remove any entries with log index >= the new entry's index.
	// These entries were created by a partitioned leader.
	for i := len(rsm.opQueue) - 1; i >= 0; i-- {
		if rsm.opQueue[i].logIndex < newEntry.logIndex {
			break
		}
		rsm.opQueue[i].ch <- OpResult{
			val: nil,
			ok:  false,
		}
		rsm.Debug("remove entry %v", rsm.opQueue[i])
		rsm.opQueue = slices.Delete(rsm.opQueue, i, i+1)
	}
	rsm.Debug("append entry %v", newEntry)
	rsm.opQueue = append(rsm.opQueue, newEntry)
	rsm.mu.Unlock()

	opRes := <-newEntry.ch

	rsm.mu.Lock()
	rsm.Debug("operation done %v", opRes)
	rsm.mu.Unlock()

	if !opRes.ok {
		return rpc.ErrWrongLeader, nil // i'm dead, try another server.
	}

	return rpc.OK, opRes.val
}

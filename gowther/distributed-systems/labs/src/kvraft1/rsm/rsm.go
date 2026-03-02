package rsm

import (
	"fmt"
	"log"
	"slices"
	"sync"
	"time"

	"6.5840/kvsrv1/rpc"
	"6.5840/labrpc"
	raft "6.5840/raft1"
	"6.5840/raftapi"
	tester "6.5840/tester1"

	"github.com/google/uuid"
)

const Debug = false

var useRaftStateMachine bool // to plug in another raft besided raft1

type Op struct {
	// Your definitions here.
	// Field names must start with capital letters,
	// otherwise RPC will break.
	Me     int
	Id     string
	Req    any
	IsNoOp bool
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

	opQueue      []OpQueueEntry
	lastSubmitAt time.Time
}

func (rsm *RSM) Debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[RSM_%d] %s\n", rsm.me, fmt.Sprintf(format, a...))
	}
}

func (rsm *RSM) handleCommand(msg raftapi.ApplyMsg) {
	rsm.mu.Lock()
	defer rsm.mu.Unlock()

	rsm.Debug("handle command, %+v", msg)

	op := msg.Command.(Op)

	if len(rsm.opQueue) > 0 && rsm.opQueue[0].logIndex == msg.CommandIndex && rsm.opQueue[0].op.Id != op.Id {
		for _, entry := range rsm.opQueue {
			entry.ch <- OpResult{
				val: nil,
				ok:  false,
			}
			rsm.Debug("remove entry %+v", entry)
		}
		rsm.opQueue = make([]OpQueueEntry, 0)
	}

	var res any
	if op.IsNoOp {
		res = nil
	} else {
		res = rsm.sm.DoOp(op.Req)
	}

	if len(rsm.opQueue) > 0 && rsm.opQueue[0].op.Id == op.Id {
		rsm.opQueue[0].ch <- OpResult{
			val: res,
			ok:  true,
		}
		rsm.Debug("remove entry %+v", rsm.opQueue[0])
		rsm.opQueue = slices.Delete(rsm.opQueue, 0, 1)
	}

	if rsm.maxraftstate > -1 && rsm.Raft().PersistBytes()*10 > rsm.maxraftstate*9 {
		rsm.Debug("create snapshot, index=%d", msg.CommandIndex)
		rsm.Raft().Snapshot(msg.CommandIndex, rsm.sm.Snapshot())
	}
}

func (rsm *RSM) handleSnapshot(msg raftapi.ApplyMsg) {
	rsm.mu.Lock()
	defer rsm.mu.Unlock()

	rsm.Debug("handle snapshot, %+v", msg)
	rsm.sm.Restore(msg.Snapshot)
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

	snapshot := persister.ReadSnapshot()
	if len(snapshot) != 0 {
		rsm.sm.Restore(snapshot)
	}

	go func() {
		for msg := range rsm.applyCh {
			rsm.mu.Lock()
			rsm.Debug("receive msg from applyCh, %+v", msg)
			rsm.mu.Unlock()

			if msg.CommandValid {
				rsm.handleCommand(msg)
			} else if msg.SnapshotValid {
				rsm.handleSnapshot(msg)
			} else {
				log.Fatalf("msg is neither a valid command nor a valid snapshot, msg=%+v\n", msg)
			}
		}

		rsm.mu.Lock()
		defer rsm.mu.Unlock()

		for _, entry := range rsm.opQueue {
			entry.ch <- OpResult{
				val: nil,
				ok:  false,
			}
			rsm.Debug("remove entry %+v", entry)
		}
		rsm.opQueue = make([]OpQueueEntry, 0)
	}()

	// periodically submit a "No-Op" command to prevent the RSM from waiting forever
	go func() {
		for {
			rsm.mu.Lock()
			lastSubmitAt := rsm.lastSubmitAt
			rsm.mu.Unlock()

			if lastSubmitAt.Add(1 * time.Second).Before(time.Now()) {
				go rsm.submit(nil, true)
			}
			time.Sleep(1 * time.Second)
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
	return rsm.submit(req, false)
}

func (rsm *RSM) submit(req any, isNoOp bool) (rpc.Err, any) {
	// Submit creates an Op structure to run a command through Raft;
	// for example: op := Op{Me: rsm.me, Id: id, Req: req}, where req
	// is the argument to Submit and id is a unique id for the op.

	rsm.mu.Lock()

	rsm.lastSubmitAt = time.Now()

	op := Op{
		Me:     rsm.me,
		Id:     uuid.NewString(),
		Req:    req,
		IsNoOp: isNoOp,
	}

	rsm.Debug("Raft.Start        - Op=%+v", op)

	index, term, isLeader := rsm.Raft().Start(op)

	rsm.Debug("Raft.Start return - Op=%+v, index=%d, term=%d, isLeader=%v", op, index, term, isLeader)

	if !isLeader {
		rsm.mu.Unlock()
		return rpc.ErrWrongLeader, nil // i'm dead, try another server.
	}

	newEntry := OpQueueEntry{
		logTerm:  term,
		logIndex: index,
		ch:       make(chan OpResult, 1),
		op:       &op,
	}

	// Wins election for term 1 and starts 3 operations
	// queue: [t1 1] -> [t1 2] -> [t1 3]
	//
	// Steps down to follower after another server wins election for term 2.
	//
	// Wins election again for term 3 and starts 1 operation.
	// [t3 2]
	//
	// Removes uncommitted entries and appends the new entry.
	// queue: [t1 1] -> [t3 2]
	for i := len(rsm.opQueue) - 1; i >= 0; i-- {
		if rsm.opQueue[i].logIndex < newEntry.logIndex {
			break
		}
		rsm.opQueue[i].ch <- OpResult{
			val: nil,
			ok:  false,
		}
		rsm.Debug("remove entry %+v", rsm.opQueue[i])
		rsm.opQueue = slices.Delete(rsm.opQueue, i, i+1)
	}
	rsm.Debug("append entry %+v", newEntry)
	rsm.opQueue = append(rsm.opQueue, newEntry)

	rsm.mu.Unlock()

	// The RSM may block the client indefinitely if the “started” log entry
	// is never applied. This can occur when no other client submits a new
	// request to the leader in the current term, and the Raft server steps
	// down from leader to follower before the command is replicated to a
	// majority of the cluster.
	opRes := <-newEntry.ch

	rsm.mu.Lock()
	defer rsm.mu.Unlock()

	rsm.Debug("operation done, entry=%+v, opRes=%+v", newEntry, opRes)

	if !opRes.ok {
		return rpc.ErrWrongLeader, nil // this operation is not committed.
	}

	return rpc.OK, opRes.val
}

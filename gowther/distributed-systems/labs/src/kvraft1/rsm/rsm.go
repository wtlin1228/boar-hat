package rsm

import (
	"fmt"
	"log"
	"sync"

	"6.5840/kvsrv1/rpc"
	"6.5840/labrpc"
	raft "6.5840/raft1"
	"6.5840/raftapi"
	tester "6.5840/tester1"
	"github.com/google/uuid"
)

var useRaftStateMachine bool // to plug in another raft besided raft1

type Op struct {
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

func (rsm *RSM) fatalf(format string, a ...interface{}) {
	if Debug {
		log.Fatalf("[RSM_%d] %s\n", rsm.me, fmt.Sprintf(format, a...))
	}
}

func (rsm *RSM) debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[RSM_%d] %s\n", rsm.me, fmt.Sprintf(format, a...))
	}
}

func (rsm *RSM) debugWithLock(format string, a ...interface{}) {
	if Debug {
		rsm.mu.Lock()
		rsm.debug(format, a...)
		rsm.mu.Unlock()
	}
}

func (rsm *RSM) rejectOp(entry *OpQueueEntry) {
	rsm.debug("reject request(%s), %+v, %+v", entry.op.Id, entry, entry.op)
	entry.ch <- OpResult{
		val: nil,
		ok:  false,
	}
}

func (rsm *RSM) clearOpQueue() {
	rsm.debug("clear op queue")
	for _, entry := range rsm.opQueue {
		rsm.rejectOp(&entry)
	}
	rsm.opQueue = make([]OpQueueEntry, 0)
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

	rsm.debug("Make RSM")

	go rsm.reader()

	return rsm
}

func (rsm *RSM) Raft() raftapi.Raft {
	return rsm.rf
}

// Your solution needs to handle an rsm leader that has called Start()
// for a request submitted with Submit() but loses its leadership before
// the request is committed to the log. One way to do this is for the
// rsm to detect that it has lost leadership, by noticing that Raft's
// term has changed or a different request has appeared at the index
// returned by Start(), and return rpc.ErrWrongLeader from Submit().
// If the ex-leader is partitioned by itself, it won't know about new
// leaders; but any client in the same partition won't be able to talk
// to a new leader either, so it's OK in this case for the server to
// wait indefinitely until the partition heals.
func (rsm *RSM) Submit(req any) (rpc.Err, any) {
	rsm.mu.Lock()

	op := Op{
		Me:  rsm.me,
		Id:  uuid.NewString(),
		Req: req,
	}

	rsm.debug("request(%s) is submitted", op.Id)

	index, term, isLeader := rsm.Raft().Start(op)

	if !isLeader {
		rsm.debug("request(%s) is ignored since it's not leader", op.Id)
		rsm.mu.Unlock()
		return rpc.ErrWrongLeader, nil // i'm dead, try another server.
	}

	newEntry := OpQueueEntry{
		logTerm:  term,
		logIndex: index,
		ch:       make(chan OpResult, 1),
		op:       &op,
	}

	// detect that it has lost leadership
	// - term has changed, or
	// - a different request has appeared at the index
	i := len(rsm.opQueue) - 1
	for i >= 0 && rsm.opQueue[i].logIndex >= newEntry.logIndex {
		rsm.rejectOp(&rsm.opQueue[i])
		i--
	}
	if i != len(rsm.opQueue)-1 {
		rsm.opQueue = rsm.opQueue[0 : i+1]
	}

	rsm.debug("request(%s) is accepted and sent to Raft, %+v", op.Id, newEntry)
	rsm.opQueue = append(rsm.opQueue, newEntry)

	rsm.mu.Unlock()

	opRes := <-newEntry.ch

	if opRes.ok {
		rsm.debugWithLock("request(%s) is processed, result=%v, %+v\n", op.Id, opRes.val, newEntry)
		return rpc.OK, opRes.val
	} else {
		rsm.debugWithLock("request(%s) is rejected, %+v\n", op.Id, newEntry)
		return rpc.ErrWrongLeader, nil
	}
}

func (rsm *RSM) reader() {
	for msg := range rsm.applyCh {
		rsm.mu.Lock()

		if msg.CommandValid {
			op := msg.Command.(Op)
			rsm.debug("receive valid command, %+v", op)

			if len(rsm.opQueue) > 0 &&
				rsm.opQueue[0].logIndex == msg.CommandIndex &&
				rsm.opQueue[0].op.Id != op.Id {
				rsm.clearOpQueue()
			}

			res := rsm.sm.DoOp(op.Req)

			rsm.debug("request(%s) is applied, result=%v", op.Id, res)

			if len(rsm.opQueue) > 0 && rsm.opQueue[0].op.Id == op.Id {
				rsm.opQueue[0].ch <- OpResult{
					val: res,
					ok:  true,
				}
				rsm.opQueue = rsm.opQueue[1:]
			}
		} else if msg.SnapshotValid {
			rsm.debug("receive valid snapshot, msg.SnapshotTerm=%d, msg.CommandIndex=%d",
				msg.SnapshotTerm, msg.CommandIndex)
			rsm.fatalf("invalid msg %+v", msg)
		} else {
			rsm.fatalf("invalid msg %+v", msg)
		}

		rsm.mu.Unlock()
	}
}

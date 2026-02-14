package kvraft

import (
	"fmt"
	"log"
	"sync"
	"sync/atomic"

	"6.5840/kvraft1/rsm"
	"6.5840/kvsrv1/rpc"
	"6.5840/labgob"
	"6.5840/labrpc"
	tester "6.5840/tester1"
)

type Entry struct {
	value   string
	version rpc.Tversion
}

type KVServer struct {
	me   int
	dead int32 // set by Kill()
	rsm  *rsm.RSM

	// Your definitions here.
	mu   sync.Mutex
	data map[string]Entry
}

func (kv *KVServer) Debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[KVServer_%d] - %s\n", kv.me, fmt.Sprintf(format, a...))
	}
}

// To type-cast req to the right type, take a look at Go's type switches or type
// assertions below:
//
// https://go.dev/tour/methods/16
// https://go.dev/tour/methods/15
func (kv *KVServer) DoOp(req any) any {
	// Your code here
	switch args := req.(type) {
	case rpc.GetArgs:
		kv.mu.Lock()
		kv.Debug("DoOp(Get(%s))", args.Key)
		entry, ok := kv.data[args.Key]
		var reply rpc.GetReply
		if !ok {
			reply = rpc.GetReply{Err: rpc.ErrNoKey}
		} else {
			reply = rpc.GetReply{
				Value:   entry.value,
				Version: entry.version,
				Err:     rpc.OK,
			}
		}
		kv.mu.Unlock()
		kv.Debug("DoOp(Get(%s)) reply=%v", args.Key, reply)
		return &reply
	case rpc.PutArgs:
		kv.mu.Lock()
		kv.Debug("DoOp(Put(%s, %s, %d))", args.Key, args.Value, args.Version)
		entry, ok := kv.data[args.Key]
		var reply rpc.PutReply
		if !ok && args.Version == 0 {
			kv.data[args.Key] = Entry{args.Value, 1}
			reply = rpc.PutReply{Err: rpc.OK}
		} else if !ok {
			reply = rpc.PutReply{Err: rpc.ErrNoKey}
		} else if entry.version == args.Version {
			kv.data[args.Key] = Entry{args.Value, entry.version + 1}
			reply = rpc.PutReply{Err: rpc.OK}
		} else {
			reply = rpc.PutReply{Err: rpc.ErrVersion}
		}
		kv.mu.Unlock()
		kv.Debug("DoOp(Put(%s, %s, %d)) reply=%v", args.Key, args.Value, args.Version, reply)
		return &reply
	default:
		log.Fatalf("DoOp should execute only Get and Put and not %T", req)
	}
	return nil
}

func (kv *KVServer) Snapshot() []byte {
	// Your code here
	return nil
}

func (kv *KVServer) Restore(data []byte) {
	// Your code here
}

func (kv *KVServer) Get(args *rpc.GetArgs, reply *rpc.GetReply) {
	kv.Debug("Get(%s)", args.Key)
	// Your code here. Use kv.rsm.Submit() to submit args
	// You can use go's type casts to turn the any return value
	// of Submit() into a GetReply: rep.(rpc.GetReply)
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = rpc.ErrWrongLeader
	} else {
		res := res.(*rpc.GetReply)
		reply.Value = res.Value
		reply.Version = res.Version
		reply.Err = res.Err
	}
}

func (kv *KVServer) Put(args *rpc.PutArgs, reply *rpc.PutReply) {
	kv.Debug("Put(%s, %s, %d)", args.Key, args.Value, args.Version)
	// Your code here. Use kv.rsm.Submit() to submit args
	// You can use go's type casts to turn the any return value
	// of Submit() into a PutReply: rep.(rpc.PutReply)
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = err
	} else {
		res := res.(*rpc.PutReply)
		reply.Err = res.Err
	}
}

// the tester calls Kill() when a KVServer instance won't
// be needed again. for your convenience, we supply
// code to set rf.dead (without needing a lock),
// and a killed() method to test rf.dead in
// long-running loops. you can also add your own
// code to Kill(). you're not required to do anything
// about this, but it may be convenient (for example)
// to suppress debug output from a Kill()ed instance.
func (kv *KVServer) Kill() {
	atomic.StoreInt32(&kv.dead, 1)
	// Your code here, if desired.
}

func (kv *KVServer) killed() bool {
	z := atomic.LoadInt32(&kv.dead)
	return z == 1
}

// StartKVServer() and MakeRSM() must return quickly, so they should
// start goroutines for any long-running work.
func StartKVServer(servers []*labrpc.ClientEnd, gid tester.Tgid, me int, persister *tester.Persister, maxraftstate int) []tester.IService {
	// call labgob.Register on structures you want
	// Go's RPC library to marshall/unmarshall.
	labgob.Register(rsm.Op{})
	labgob.Register(rpc.PutArgs{})
	labgob.Register(rpc.GetArgs{})

	kv := &KVServer{me: me}

	kv.rsm = rsm.MakeRSM(servers, me, persister, maxraftstate, kv)
	// You may need initialization code here.
	kv.data = make(map[string]Entry)
	return []tester.IService{kv, kv.rsm.Raft()}
}

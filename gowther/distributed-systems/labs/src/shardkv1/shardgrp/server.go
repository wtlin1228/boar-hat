package shardgrp

import (
	"bytes"
	"fmt"
	"log"
	"sync"
	"sync/atomic"

	"6.5840/kvraft1/rsm"
	"6.5840/kvsrv1/rpc"
	"6.5840/labgob"
	"6.5840/labrpc"
	"6.5840/shardkv1/shardcfg"
	"6.5840/shardkv1/shardgrp/shardrpc"
	tester "6.5840/tester1"
)

type Shard struct {
	freeze bool
	num    shardcfg.Tnum
}

type Entry struct {
	Value   string
	Version rpc.Tversion
}

type KVServer struct {
	me   int
	dead int32 // set by Kill()
	rsm  *rsm.RSM
	gid  tester.Tgid

	// Your code here
	mu   sync.Mutex
	data map[string]Entry

	// each group can serve more than one shard
	shardMap map[shardcfg.Tshid]Shard
}

func (kv *KVServer) fatalf(format string, a ...interface{}) {
	log.Fatalf("[shardgrp_%d::KVServer_%d] %s\n", kv.gid, kv.me, fmt.Sprintf(format, a...))
}

func (kv *KVServer) debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[shardgrp_%d::KVServer_%d] %s\n", kv.gid, kv.me, fmt.Sprintf(format, a...))
	}
}

func (kv *KVServer) getShardState(shid shardcfg.Tshid) []byte {
	shardState := make(map[string]Entry)
	for k, v := range kv.data {
		if shardcfg.Key2Shard(k) == shid {
			shardState[k] = v
		}
	}
	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	e.Encode(shardState)
	return w.Bytes()
}

func (kv *KVServer) installShardState(data []byte) {
	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	var shardState map[string]Entry
	if d.Decode(&shardState) != nil {
		kv.fatalf("installShardState - failed to decode the shard data")
	} else {
		for k, v := range shardState {
			kv.data[k] = v
		}
	}
}

func (kv *KVServer) DoOp(req any) any {
	// Your code here
	kv.mu.Lock()
	defer kv.mu.Unlock()

	switch args := req.(type) {
	case rpc.GetArgs:
		shard, exist := kv.shardMap[shardcfg.Key2Shard(args.Key)]
		entry, ok := kv.data[args.Key]
		if !exist || shard.freeze {
			return &rpc.GetReply{Err: rpc.ErrWrongGroup}
		} else if !ok {
			return &rpc.GetReply{Err: rpc.ErrNoKey}
		} else {
			return &rpc.GetReply{
				Value:   entry.Value,
				Version: entry.Version,
				Err:     rpc.OK,
			}
		}

	case rpc.PutArgs:
		shard, exist := kv.shardMap[shardcfg.Key2Shard(args.Key)]
		entry, ok := kv.data[args.Key]
		if !exist || shard.freeze {
			return &rpc.PutReply{Err: rpc.ErrWrongGroup}
		} else if !ok && args.Version == 0 {
			kv.data[args.Key] = Entry{args.Value, 1}
			return &rpc.PutReply{Err: rpc.OK}
		} else if !ok {
			return &rpc.PutReply{Err: rpc.ErrNoKey}
		} else if entry.Version == args.Version {
			kv.data[args.Key] = Entry{args.Value, entry.Version + 1}
			return &rpc.PutReply{Err: rpc.OK}
		} else {
			return &rpc.PutReply{Err: rpc.ErrVersion}
		}

	case shardrpc.FreezeShardArgs:
		shard, exist := kv.shardMap[args.Shard]
		if !exist {
			return &shardrpc.FreezeShardReply{Err: rpc.ErrWrongGroup}
		} else if args.Num == 0 || args.Num < shard.num {
			return &shardrpc.FreezeShardReply{Err: rpc.ErrVersion}
		}
		shard.num = args.Num
		shard.freeze = true
		return &shardrpc.FreezeShardReply{
			State: kv.getShardState(args.Shard),
			Num:   args.Num,
			Err:   rpc.OK,
		}

	case shardrpc.InstallShardArgs:
		if args.Num == 0 {
			// initialize phase
			if len(args.State) != 0 || len(kv.data) != 0 {
				return &shardrpc.InstallShardReply{Err: rpc.ErrVersion}
			}
			kv.shardMap[args.Shard] = Shard{freeze: false, num: 0}
			return &shardrpc.InstallShardReply{Err: rpc.OK}
		}
		shard, exist := kv.shardMap[args.Shard]
		if !exist {
			return &shardrpc.InstallShardReply{Err: rpc.ErrWrongGroup}
		} else if args.Num < shard.num {
			return &shardrpc.InstallShardReply{Err: rpc.ErrVersion}
		}
		kv.installShardState(args.State)
		shard.num = args.Num
		return &shardrpc.InstallShardReply{Err: rpc.OK}

	case shardrpc.DeleteShardArgs:
		kv.fatalf("not implemented yet")

	default:
		kv.fatalf("DoOp - unexpected operation %T", req)
	}
	return nil
}

func (kv *KVServer) Snapshot() []byte {
	// Your code here
	kv.mu.Lock()
	defer kv.mu.Unlock()

	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	e.Encode(kv.data)
	return w.Bytes()
}

func (kv *KVServer) Restore(data []byte) {
	// Your code here
	kv.mu.Lock()
	defer kv.mu.Unlock()

	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	var decodedData map[string]Entry
	if d.Decode(&decodedData) != nil {
		log.Fatalln("shardgrp::Server::Restore - fail to decode the restore data")
	} else {
		kv.data = decodedData
	}
}

func (kv *KVServer) Get(args *rpc.GetArgs, reply *rpc.GetReply) {
	// Your code here
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
	// Your code here
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = err
	} else {
		res := res.(*rpc.PutReply)
		reply.Err = res.Err
	}
}

// Freeze the specified shard (i.e., reject future Get/Puts for this
// shard) and return the key/values stored in that shard.
func (kv *KVServer) FreezeShard(args *shardrpc.FreezeShardArgs, reply *shardrpc.FreezeShardReply) {
	// Your code here
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = err
	} else {
		res := res.(*shardrpc.FreezeShardReply)
		reply.Num = res.Num
		reply.State = res.State
		reply.Err = res.Err
	}
}

// Install the supplied state for the specified shard.
func (kv *KVServer) InstallShard(args *shardrpc.InstallShardArgs, reply *shardrpc.InstallShardReply) {
	// Your code here
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = err
	} else {
		res := res.(*shardrpc.InstallShardReply)
		reply.Err = res.Err
	}
}

// Delete the specified shard.
func (kv *KVServer) DeleteShard(args *shardrpc.DeleteShardArgs, reply *shardrpc.DeleteShardReply) {
	// Your code here
	err, res := kv.rsm.Submit(*args)
	if err == rpc.ErrWrongLeader {
		reply.Err = err
	} else {
		res := res.(*shardrpc.DeleteShardReply)
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

// StartShardServerGrp starts a server for shardgrp `gid`.
//
// StartShardServerGrp() and MakeRSM() must return quickly, so they should
// start goroutines for any long-running work.
func StartServerShardGrp(servers []*labrpc.ClientEnd, gid tester.Tgid, me int, persister *tester.Persister, maxraftstate int) []tester.IService {
	// call labgob.Register on structures you want
	// Go's RPC library to marshall/unmarshall.
	labgob.Register(rpc.PutArgs{})
	labgob.Register(rpc.GetArgs{})
	labgob.Register(shardrpc.FreezeShardArgs{})
	labgob.Register(shardrpc.InstallShardArgs{})
	labgob.Register(shardrpc.DeleteShardArgs{})
	labgob.Register(rsm.Op{})

	kv := &KVServer{gid: gid, me: me}
	kv.rsm = rsm.MakeRSM(servers, me, persister, maxraftstate, kv)
	kv.data = make(map[string]Entry)
	kv.shardMap = make(map[shardcfg.Tshid]Shard)

	snapshot := persister.ReadSnapshot()
	if len(snapshot) != 0 {
		kv.Restore(snapshot)
	}

	// Your code here
	kv.debug("StartServerShardGrp")

	return []tester.IService{kv, kv.rsm.Raft()}
}

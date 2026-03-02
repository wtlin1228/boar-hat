package kvsrv

import (
	"log"
	"sync"

	"6.5840/kvsrv1/rpc"
	"6.5840/labrpc"
	tester "6.5840/tester1"
)

const Debug = false

func DPrintf(format string, a ...interface{}) (n int, err error) {
	if Debug {
		log.Printf(format, a...)
	}
	return
}

type Entry struct {
	value   string
	version rpc.Tversion
}

type KVServer struct {
	mu sync.Mutex

	// Your definitions here.
	data map[string]Entry
}

func MakeKVServer() *KVServer {
	kv := &KVServer{
		sync.Mutex{},
		make(map[string]Entry),
	}
	// Your code here.
	return kv
}

// Get returns the value and version for args.Key, if args.Key
// exists. Otherwise, Get returns ErrNoKey.
func (kv *KVServer) Get(args *rpc.GetArgs, reply *rpc.GetReply) {
	// Your code here.
	kv.mu.Lock()
	entry, ok := kv.data[args.Key]
	kv.mu.Unlock()
	if !ok {
		reply.Err = rpc.ErrNoKey
	} else {
		reply.Value = entry.value
		reply.Version = entry.version
		reply.Err = rpc.OK
	}
}

// Update the value for a key if args.Version matches the version of
// the key on the server. If versions don't match, return ErrVersion.
// If the key doesn't exist, Put installs the value if the
// args.Version is 0, and returns ErrNoKey otherwise.
func (kv *KVServer) Put(args *rpc.PutArgs, reply *rpc.PutReply) {
	// Your code here.
	kv.mu.Lock()
	entry, ok := kv.data[args.Key]
	if !ok && args.Version == 0 {
		kv.data[args.Key] = Entry{args.Value, 1}
		reply.Err = rpc.OK
	} else if !ok {
		reply.Err = rpc.ErrNoKey
	} else if entry.version == args.Version {
		kv.data[args.Key] = Entry{args.Value, entry.version + 1}
		reply.Err = rpc.OK
	} else {
		reply.Err = rpc.ErrVersion
	}
	kv.mu.Unlock()
}

// You can ignore Kill() for this lab
func (kv *KVServer) Kill() {
}

// You can ignore all arguments; they are for replicated KVservers
func StartKVServer(ends []*labrpc.ClientEnd, gid tester.Tgid, srv int, persister *tester.Persister) []tester.IService {
	kv := MakeKVServer()
	return []tester.IService{kv}
}

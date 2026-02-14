package kvraft

import (
	"fmt"
	"log"
	"sync"
	"time"

	"6.5840/kvsrv1/rpc"
	kvtest "6.5840/kvtest1"
	tester "6.5840/tester1"
)

type Clerk struct {
	clnt    *tester.Clnt
	servers []string
	// You will have to modify this struct.
	mu     sync.Mutex
	leader int
}

func (ck *Clerk) Debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[Clerk] - %s\n", fmt.Sprintf(format, a...))
	}
}

func MakeClerk(clnt *tester.Clnt, servers []string) kvtest.IKVClerk {
	ck := &Clerk{clnt: clnt, servers: servers, leader: 0}
	// You'll have to add code here.
	return ck
}

func (ck *Clerk) changeLeader(leader int) {
	ck.mu.Lock()
	defer ck.mu.Unlock()
	if ck.leader != leader {
		ck.leader = leader
	}
}

// Get fetches the current value and version for a key.  It returns
// ErrNoKey if the key does not exist. It keeps trying forever in the
// face of all other errors.
//
// You can send an RPC to server i with code like this:
// ok := ck.clnt.Call(ck.servers[i], "KVServer.Get", &args, &reply)
//
// The types of args and reply (including whether they are pointers)
// must match the declared types of the RPC handler function's
// arguments. Additionally, reply must be passed as a pointer.
func (ck *Clerk) Get(key string) (string, rpc.Tversion, rpc.Err) {
	// You will have to modify this function.

	ck.mu.Lock()
	ck.Debug("Get(%s) -> server_%d", key, ck.leader)
	ck.mu.Unlock()

	for {
		args := rpc.GetArgs{Key: key}
		reply := rpc.GetReply{}

		ck.mu.Lock()
		leader := ck.leader
		ck.mu.Unlock()

		ok := ck.clnt.Call(ck.servers[leader], "KVServer.Get", &args, &reply)

		if !ok {
			ck.changeLeader((leader + 1) % len(ck.servers)) // to prevent partition
			time.Sleep(100 * time.Millisecond)
			continue
		}

		if reply.Err == rpc.ErrWrongLeader {
			ck.changeLeader((leader + 1) % len(ck.servers))
			time.Sleep(2 * time.Millisecond) // to prevent excessive RPC calls
			continue
		}

		return reply.Value, reply.Version, reply.Err
	}
}

// Put updates key with value only if the version in the
// request matches the version of the key at the server.  If the
// versions numbers don't match, the server should return
// ErrVersion.  If Put receives an ErrVersion on its first RPC, Put
// should return ErrVersion, since the Put was definitely not
// performed at the server. If the server returns ErrVersion on a
// resend RPC, then Put must return ErrMaybe to the application, since
// its earlier RPC might have been processed by the server successfully
// but the response was lost, and the the Clerk doesn't know if
// the Put was performed or not.
//
// You can send an RPC to server i with code like this:
// ok := ck.clnt.Call(ck.servers[i], "KVServer.Put", &args, &reply)
//
// The types of args and reply (including whether they are pointers)
// must match the declared types of the RPC handler function's
// arguments. Additionally, reply must be passed as a pointer.
func (ck *Clerk) Put(key string, value string, version rpc.Tversion) rpc.Err {
	// You will have to modify this function.

	ck.Debug("Put(%s, %s, %d) -> server_%d", key, value, version, ck.leader)

	retryTimes := 0
	for {
		args := rpc.PutArgs{Key: key, Value: value, Version: version}
		reply := rpc.PutReply{}

		ck.mu.Lock()
		leader := ck.leader
		ck.mu.Unlock()

		ok := ck.clnt.Call(ck.servers[leader], "KVServer.Put", &args, &reply)

		if !ok {
			ck.changeLeader((leader + 1) % len(ck.servers)) // to prevent partition
			time.Sleep(100 * time.Millisecond)
			retryTimes += 1
			continue
		}

		if reply.Err == rpc.ErrWrongLeader {
			ck.changeLeader((leader + 1) % len(ck.servers))
			time.Sleep(2 * time.Millisecond) // to prevent excessive RPC calls
			continue
		}

		if retryTimes > 0 && reply.Err == rpc.ErrVersion {
			reply.Err = rpc.ErrMaybe
		}

		return reply.Err
	}
}

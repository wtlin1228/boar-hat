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

const (
	Timeout  = 1 * time.Second
	Throttle = 2 * time.Millisecond
)

type Clerk struct {
	clnt    *tester.Clnt
	servers []string
	// You will have to modify this struct.
	mu     sync.Mutex
	prefer int
}

func (ck *Clerk) debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[Clerk] %s\n", fmt.Sprintf(format, a...))
	}
}

func MakeClerk(clnt *tester.Clnt, servers []string) kvtest.IKVClerk {
	ck := &Clerk{clnt: clnt, servers: servers}
	// You'll have to add code here.
	return ck
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
	ck.debug("Get(key=%s)", key)

	type result struct {
		serverId int
		ok       bool
		reply    rpc.GetReply
	}

	t := time.NewTimer(Timeout)
	defer t.Stop()

	done := make(chan result, len(ck.servers))

	ck.mu.Lock()
	prefer := ck.prefer
	ck.mu.Unlock()

	var r result
	for offset := 0; ; offset++ {
		id := (offset + prefer) % len(ck.servers)
		go func() {
			ck.debug("Get(key=%s) -> server %d", key, id)
			args := rpc.GetArgs{Key: key}
			reply := rpc.GetReply{}
			ok := ck.clnt.Call(ck.servers[id], "KVServer.Get", &args, &reply)
			done <- result{id, ok, reply}
		}()

		select {
		case <-t.C:
		case r = <-done:
			if r.ok && r.reply.Err != rpc.ErrWrongLeader {
				goto Done
			}
		}
		t.Reset(Timeout)
		time.Sleep(Throttle) // to prevent excessive RPC calls
	}

Done:
	ck.mu.Lock()
	ck.debug("Get(key=%s) -> server %d, reply=%+v", key, r.serverId, r.reply)
	ck.prefer = r.serverId
	ck.mu.Unlock()
	return r.reply.Value, r.reply.Version, r.reply.Err
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
	ck.debug("Put(key=%s, value=%s, version=%v)", key, value, version)

	type result struct {
		serverId int
		ok       bool
		reply    rpc.PutReply
	}

	t := time.NewTimer(Timeout)
	defer t.Stop()

	done := make(chan result, len(ck.servers))

	ck.mu.Lock()
	prefer := ck.prefer
	ck.mu.Unlock()

	retryCount := 0
	var r result
	for offset := 0; ; offset++ {
		id := (offset + prefer) % len(ck.servers)
		go func() {
			ck.debug("Put(key=%s, value=%s, version=%v) -> server %d", key, value, version, id)
			args := rpc.PutArgs{Key: key, Value: value, Version: version}
			reply := rpc.PutReply{}
			ok := ck.clnt.Call(ck.servers[id], "KVServer.Put", &args, &reply)
			done <- result{id, ok, reply}
		}()

		select {
		case <-t.C:
		case r = <-done:
			if r.ok && r.reply.Err != rpc.ErrWrongLeader {
				if r.reply.Err == rpc.ErrVersion && retryCount > 0 {
					r.reply.Err = rpc.ErrMaybe
				}
				goto Done
			}
		}
		t.Reset(Timeout)
		retryCount += 1
		time.Sleep(Throttle) // to prevent excessive RPC calls
	}

Done:
	ck.mu.Lock()
	ck.debug("Put(key=%s, value=%s, version=%v) -> server %d, reply=%+v", key, value, version, r.serverId, r.reply)
	ck.prefer = r.serverId
	ck.mu.Unlock()
	return r.reply.Err
}

package shardgrp

import (
	"sync"
	"time"

	"6.5840/kvsrv1/rpc"
	"6.5840/shardkv1/shardcfg"
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

func MakeClerk(clnt *tester.Clnt, servers []string) *Clerk {
	ck := &Clerk{clnt: clnt, servers: servers}
	return ck
}

func (ck *Clerk) Get(key string) (string, rpc.Tversion, rpc.Err) {
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
	ck.prefer = r.serverId
	ck.mu.Unlock()
	return r.reply.Value, r.reply.Version, r.reply.Err
}

func (ck *Clerk) Put(key string, value string, version rpc.Tversion) rpc.Err {
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
	ck.prefer = r.serverId
	ck.mu.Unlock()
	return r.reply.Err
}

func (ck *Clerk) FreezeShard(s shardcfg.Tshid, num shardcfg.Tnum) ([]byte, rpc.Err) {
	// Your code here
	return nil, ""
}

func (ck *Clerk) InstallShard(s shardcfg.Tshid, state []byte, num shardcfg.Tnum) rpc.Err {
	// Your code here
	return ""
}

func (ck *Clerk) DeleteShard(s shardcfg.Tshid, num shardcfg.Tnum) rpc.Err {
	// Your code here
	return ""
}

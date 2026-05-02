package shardkv

//
// client code to talk to a sharded key/value service.
//
// the client uses the shardctrler to query for the current
// configuration and find the assignment of shards (keys) to groups,
// and then talks to the group that holds the key's shard.
//

import (
	"fmt"
	"log"

	"6.5840/kvsrv1/rpc"
	kvtest "6.5840/kvtest1"
	"6.5840/shardkv1/shardcfg"
	"6.5840/shardkv1/shardctrler"
	"6.5840/shardkv1/shardgrp"
	tester "6.5840/tester1"
)

type Clerk struct {
	clnt *tester.Clnt
	sck  *shardctrler.ShardCtrler
	// You will have to modify this struct.
}

func (ck *Clerk) fatalf(format string, a ...interface{}) {
	log.Fatalf("[Clerk] %s\n", fmt.Sprintf(format, a...))
}

func (ck *Clerk) debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[Clerk] %s\n", fmt.Sprintf(format, a...))
	}
}

// The tester calls MakeClerk and passes in a shardctrler so that
// client can call it's Query method
func MakeClerk(clnt *tester.Clnt, sck *shardctrler.ShardCtrler) kvtest.IKVClerk {
	ck := &Clerk{
		clnt: clnt,
		sck:  sck,
	}
	// You'll have to add code here.
	return ck
}

// Get a key from a shardgrp.  You can use shardcfg.Key2Shard(key) to
// find the shard responsible for the key and ck.sck.Query() to read
// the current configuration and lookup the servers in the group
// responsible for key.  You can make a clerk for that group by
// calling shardgrp.MakeClerk(ck.clnt, servers).
func (ck *Clerk) Get(key string) (string, rpc.Tversion, rpc.Err) {
	// You will have to modify this function.
	return ck.makeShardgrpClerk(key).Get(key)
}

// Put a key to a shard group.
func (ck *Clerk) Put(key string, value string, version rpc.Tversion) rpc.Err {
	// You will have to modify this function.
	return ck.makeShardgrpClerk(key).Put(key, value, version)
}

func (ck *Clerk) makeShardgrpClerk(key string) *shardgrp.Clerk {
	shid := shardcfg.Key2Shard(key)
	config := ck.sck.Query()
	_, servers, ok := config.GidServers(shid)
	if !ok {
		ck.fatalf("makeShardgrpClerk - failed to get servers of shard_%d", shid)
	}
	return shardgrp.MakeClerk(ck.clnt, servers)
}

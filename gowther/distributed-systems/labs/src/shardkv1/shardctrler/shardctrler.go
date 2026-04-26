package shardctrler

//
// Shardctrler with InitConfig, Query, and ChangeConfigTo methods
//

import (
	"log"

	kvsrv "6.5840/kvsrv1"
	"6.5840/kvsrv1/rpc"
	kvtest "6.5840/kvtest1"
	"6.5840/shardkv1/shardcfg"
	tester "6.5840/tester1"
)

const (
	ConfigKey string = "SHARD_CONFIG"
)

// ShardCtrler for the controller and kv clerk.
type ShardCtrler struct {
	clnt *tester.Clnt
	kvtest.IKVClerk

	killed int32 // set by Kill()

	// Your data here.
	configVersion rpc.Tversion
}

// Make a ShardCltler, which stores its state in a kvsrv.
func MakeShardCtrler(clnt *tester.Clnt) *ShardCtrler {
	sck := &ShardCtrler{clnt: clnt}
	srv := tester.ServerName(tester.GRP0, 0)
	sck.IKVClerk = kvsrv.MakeClerk(clnt, srv)
	// Your code here.
	return sck
}

// The tester calls InitController() before starting a new
// controller. In part A, this method doesn't need to do anything. In
// B and C, this method implements recovery.
func (sck *ShardCtrler) InitController() {
}

// Called once by the tester to supply the first configuration.  You
// can marshal ShardConfig into a string using shardcfg.String(), and
// then Put it in the kvsrv for the controller at version 0.  You can
// pick the key to name the configuration.  The initial configuration
// lists shardgrp shardcfg.Gid1 for all shards.
func (sck *ShardCtrler) InitConfig(cfg *shardcfg.ShardConfig) {
	// Your code here
	err := sck.IKVClerk.Put(ConfigKey, cfg.String(), 0)
	if err == rpc.OK {
		sck.configVersion = 1
	} else {
		log.Fatalf("ShardCtrler::InitConfig - failed, err=%s\n", err)
	}
}

// Called by the tester to ask the controller to change the
// configuration from the current one to new.  While the controller
// changes the configuration it may be superseded by another
// controller.
func (sck *ShardCtrler) ChangeConfigTo(new *shardcfg.ShardConfig) {
	// Your code here.
	err := sck.IKVClerk.Put(ConfigKey, new.String(), sck.configVersion)
	if err == rpc.OK {
		sck.configVersion += 1
	} else {
		log.Fatalf("ShardCtrler::ChangeConfigTo - failed, err=%s\n", err)
	}
}

// Return the current configuration
func (sck *ShardCtrler) Query() *shardcfg.ShardConfig {
	// Your code here.
	value, version, err := sck.IKVClerk.Get(ConfigKey)
	if err == rpc.OK {
		sck.configVersion = version
		return shardcfg.FromString(value)
	}
	log.Fatalf("ShardCtrler::Query - failed, err=%s\n", err)
	return nil
}

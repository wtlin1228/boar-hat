package shardctrler

//
// Shardctrler with InitConfig, Query, and ChangeConfigTo methods
//

import (
	"fmt"
	"log"
	"sync"

	kvsrv "6.5840/kvsrv1"
	"6.5840/kvsrv1/rpc"
	kvtest "6.5840/kvtest1"
	"6.5840/shardkv1/shardcfg"
	"6.5840/shardkv1/shardgrp"
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

func (sck *ShardCtrler) fatalf(format string, a ...interface{}) {
	log.Fatalf("[ShardCtrler] %s\n", fmt.Sprintf(format, a...))
}

func (sck *ShardCtrler) debug(format string, a ...interface{}) {
	if Debug {
		log.Printf("[ShardCtrler] %s\n", fmt.Sprintf(format, a...))
	}
}

// Make a ShardCltler, which stores its state in a kvsrv.
func MakeShardCtrler(clnt *tester.Clnt) *ShardCtrler {
	sck := &ShardCtrler{clnt: clnt}
	srv := tester.ServerName(tester.GRP0, 0)
	sck.IKVClerk = kvsrv.MakeClerk(clnt, srv)
	// Your code here.
	sck.debug("MakeShardCtrler")

	return sck
}

// The tester calls InitController() before starting a new
// controller. In part A, this method doesn't need to do anything. In
// B and C, this method implements recovery.
func (sck *ShardCtrler) InitController() {
	sck.debug("InitController")

}

// Called once by the tester to supply the first configuration.  You
// can marshal ShardConfig into a string using shardcfg.String(), and
// then Put it in the kvsrv for the controller at version 0.  You can
// pick the key to name the configuration.  The initial configuration
// lists shardgrp shardcfg.Gid1 for all shards.
func (sck *ShardCtrler) InitConfig(cfg *shardcfg.ShardConfig) {
	sck.debug("InitConfig")

	sck.initShards(cfg)

	err := sck.IKVClerk.Put(ConfigKey, cfg.String(), 0)
	if err == rpc.OK {
		sck.configVersion = 1
	} else {
		sck.fatalf("InitConfig - failed to put the new config\n")
	}
}

// Called by the tester to ask the controller to change the
// configuration from the current one to new.  While the controller
// changes the configuration it may be superseded by another
// controller.
func (sck *ShardCtrler) ChangeConfigTo(new *shardcfg.ShardConfig) {
	sck.debug("ChangeConfigTo, new=%+v", new)

	err := sck.IKVClerk.Put(ConfigKey, new.String(), sck.configVersion)
	if err == rpc.OK {
		sck.configVersion += 1
	} else {
		sck.fatalf("ChangeConfigTo - failed, err=%s\n", err)
	}
}

// Return the current configuration
func (sck *ShardCtrler) Query() *shardcfg.ShardConfig {
	sck.debug("Query")

	value, version, err := sck.IKVClerk.Get(ConfigKey)
	if err == rpc.OK {
		sck.configVersion = version
		return shardcfg.FromString(value)
	}
	sck.fatalf("Query - failed, err=%s\n", err)
	return nil
}

func (sck *ShardCtrler) makeShardgrpClerk(config *shardcfg.ShardConfig, shid shardcfg.Tshid) *shardgrp.Clerk {
	_, servers, ok := config.GidServers(shid)
	if !ok {
		sck.fatalf("makeShardgrpClerk - failed to call config.GidServers(%d)", shid)
	}
	return shardgrp.MakeClerk(sck.clnt, servers)
}

func (sck *ShardCtrler) initShards(cfg *shardcfg.ShardConfig) {
	sck.debug("initShards")

	var wg sync.WaitGroup
	errCh := make(chan struct{}, len(cfg.Shards))

	for shid := range len(cfg.Shards) {
		wg.Add(1)

		go func(shid shardcfg.Tshid) {
			defer wg.Done()

			shardgrpClerk := sck.makeShardgrpClerk(cfg, shid)
			err := shardgrpClerk.InitShard(shid, 0)
			if err != rpc.OK {
				sck.debug("initShards - initialize shard_%d failed", shid)
				errCh <- struct{}{}
			} else {
				sck.debug("initShards - initialize shard_%d succeeded", shid)
			}
		}(shardcfg.Tshid(shid))
	}

	wg.Wait()
	close(errCh)

	for range errCh {
		// not fatal here since the first test case doesn't make the KVServer
		sck.debug("initShards - failed to init shards\n")
	}
}

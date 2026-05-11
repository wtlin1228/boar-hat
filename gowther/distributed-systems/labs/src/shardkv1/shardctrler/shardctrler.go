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
	CurrentConfigKey string = "CURRENT_CONFIG"
	NextConfigKey    string = "NEXT_CONFIG"
)

// ShardCtrler for the controller and kv clerk.
type ShardCtrler struct {
	clnt *tester.Clnt
	kvtest.IKVClerk

	killed int32 // set by Kill()

	// Your data here.
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

	_, currentVersion := sck.query(CurrentConfigKey)
	nextCfg, nextVersion := sck.query(NextConfigKey)

	if currentVersion != nextVersion {
		sck.changeConfigTo(nextCfg, true)
	}
}

// Called once by the tester to supply the first configuration.  You
// can marshal ShardConfig into a string using shardcfg.String(), and
// then Put it in the kvsrv for the controller at version 0.  You can
// pick the key to name the configuration.  The initial configuration
// lists shardgrp shardcfg.Gid1 for all shards.
func (sck *ShardCtrler) InitConfig(cfg *shardcfg.ShardConfig) {
	sck.debug("InitConfig")

	sck.put(NextConfigKey, cfg, 0)
	sck.initShards(cfg)
	sck.put(CurrentConfigKey, cfg, 0)

	sck.debug("InitConfig succeeded")
}

// Called by the tester to ask the controller to change the
// configuration from the current one to new.  While the controller
// changes the configuration it may be superseded by another
// controller.
func (sck *ShardCtrler) ChangeConfigTo(new *shardcfg.ShardConfig) {
	sck.changeConfigTo(new, false)
}

// Return the current configuration
func (sck *ShardCtrler) Query() *shardcfg.ShardConfig {
	sck.debug("Query")
	cfg, version := sck.query(CurrentConfigKey)
	sck.debug("Query - success, version=%d, cfg=%s", version, cfg.String())
	return cfg
}

func (sck *ShardCtrler) changeConfigTo(new *shardcfg.ShardConfig, isRetry bool) {
	sck.debug("changeConfigTo - new=%+v", new)

	cfg, version := sck.query(CurrentConfigKey)
	if !isRetry {
		sck.put(NextConfigKey, new, version)
	}

	var wg sync.WaitGroup
	errCh := make(chan struct{}, len(cfg.Shards))

	for shid := range len(cfg.Shards) {
		shid := shardcfg.Tshid(shid)
		oldServers := cfg.Groups[cfg.Shards[shid]]
		newServers := new.Groups[new.Shards[shid]]
		if !equalUnordered(oldServers, newServers) {
			wg.Add(1)

			go func(shid shardcfg.Tshid) {
				defer wg.Done()

				oldShardgrpClerk := sck.makeShardgrpClerk(cfg, shid)
				newShardgrpClerk := sck.makeShardgrpClerk(new, shid)
				shardData, err := oldShardgrpClerk.FreezeShard(shid, new.Num)
				if err != rpc.OK {
					sck.debug("changeConfigTo - failed to freeze shard_%d", shid)
					errCh <- struct{}{}
					return
				}
				err = newShardgrpClerk.InstallShard(shid, shardData, new.Num)
				if err != rpc.OK {
					sck.debug("changeConfigTo - failed to install shard_%d", shid)
					errCh <- struct{}{}
					return
				}
				err = oldShardgrpClerk.DeleteShard(shid, new.Num)
				if err != rpc.OK {
					sck.debug("changeConfigTo - failed to delete shard_%d", shid)
					errCh <- struct{}{}
					return
				}
			}(shid)
		}
	}

	wg.Wait()
	close(errCh)

	for range errCh {
		sck.debug("changeConfigTo - failed to apply the new config")
		return
	}
	sck.debug("changeConfigTo - apply the new config succeeded")

	sck.put(CurrentConfigKey, new, version)

	sck.debug("changeConfigTo - succeeded, new=%+v", new)
}

func (sck *ShardCtrler) query(key string) (*shardcfg.ShardConfig, rpc.Tversion) {
	value, version, err := sck.IKVClerk.Get(key)
	if err == rpc.OK {
		return shardcfg.FromString(value), version
	}
	sck.fatalf("query(%s) - failed, err=%s\n", key, err)
	return nil, 0
}

func (sck *ShardCtrler) put(key string, config *shardcfg.ShardConfig, version rpc.Tversion) {
	configString := config.String()
	sck.IKVClerk.Put(key, configString, version)
	for {
		currentConfig, currentVersion := sck.query(key)
		if currentConfig.String() == configString && currentVersion == version+1 {
			break
		}
		sck.debug("put(%s) - failed to put new config, try again", key)
		sck.IKVClerk.Put(key, configString, version)
	}
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
			err := shardgrpClerk.InstallShard(shid, nil, 0)
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
		// NOTE: the first test case doesn't start any KVServer
		sck.debug("initShards - failed to init shards")
	}
}

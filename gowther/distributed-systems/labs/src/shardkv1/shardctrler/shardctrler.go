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
	sck.applyNextConfig()
}

// Called once by the tester to supply the first configuration.  You
// can marshal ShardConfig into a string using shardcfg.String(), and
// then Put it in the kvsrv for the controller at version 0.  You can
// pick the key to name the configuration.  The initial configuration
// lists shardgrp shardcfg.Gid1 for all shards.
func (sck *ShardCtrler) InitConfig(cfg *shardcfg.ShardConfig) {
	sck.debug("InitConfig")

	initCfg := cfg.Copy()
	initCfg.Num = 0
	sck.put(NextConfigKey, initCfg)
	sck.put(CurrentConfigKey, initCfg)

	sck.put(NextConfigKey, cfg)
	sck.put(CurrentConfigKey, cfg)
	sck.initShards(cfg)

	sck.debug("InitConfig succeeded")
}

// Called by the tester to ask the controller to change the
// configuration from the current one to new.  While the controller
// changes the configuration it may be superseded by another
// controller.
func (sck *ShardCtrler) ChangeConfigTo(new *shardcfg.ShardConfig) {
	ok := sck.put(NextConfigKey, new)
	if !ok {
		sck.debug("ChangeConfigTo - rejected, new.Num=%d, new=%+v", new.Num, new)
		return
	}

	sck.debug("ChangeConfigTo - new.Num=%d, new=%+v", new.Num, new)
	sck.applyNextConfig()
}

// Return the current configuration
func (sck *ShardCtrler) Query() *shardcfg.ShardConfig {
	sck.debug("Query")
	cfg, version := sck.query(CurrentConfigKey)
	sck.debug("Query - success, version=%d, cfg=%s", version, cfg.String())
	return cfg
}

func (sck *ShardCtrler) applyNextConfig() {
	next, nextVersion := sck.query(NextConfigKey)
	curr, currVersion := sck.query(CurrentConfigKey)

	sck.debug("applyNextConfig - currVersion=%d, nextVersion=%d", currVersion, nextVersion)
	if nextVersion == currVersion {
		sck.debug("applyNextConfig - early return, next config has already been applied and write to current config")
	} else if nextVersion != currVersion+1 {
		sck.fatalf("applyNextConfig - nextVersion can only be equal or one version ahead the currVersion")
	}

	var wg sync.WaitGroup
	errCh := make(chan struct{}, len(curr.Shards))

	for shid := range len(curr.Shards) {
		shid := shardcfg.Tshid(shid)
		oldServers := curr.Groups[curr.Shards[shid]]
		newServers := next.Groups[next.Shards[shid]]
		if !equalUnordered(oldServers, newServers) {
			wg.Add(1)

			go func(shid shardcfg.Tshid) {
				defer wg.Done()

				oldShardgrpClerk := sck.makeShardgrpClerk(curr, shid)
				newShardgrpClerk := sck.makeShardgrpClerk(next, shid)
				shardData, err := oldShardgrpClerk.FreezeShard(shid, next.Num)
				if err == rpc.ErrWrongGroup {
					// The shard is not in this group anymore, implying that it has
					// been deleted so we can skip the install and delete.
					return
				} else if err != rpc.OK {
					sck.debug("applyNextConfig - failed to freeze shard_%d", shid)
					errCh <- struct{}{}
					return
				}
				err = newShardgrpClerk.InstallShard(shid, shardData, next.Num)
				if err != rpc.OK {
					sck.debug("applyNextConfig - failed to install shard_%d", shid)
					errCh <- struct{}{}
					return
				}
				err = oldShardgrpClerk.DeleteShard(shid, next.Num)
				if err != rpc.OK {
					sck.debug("applyNextConfig - failed to delete shard_%d", shid)
					errCh <- struct{}{}
					return
				}
			}(shid)
		}
	}

	wg.Wait()
	close(errCh)
	for range errCh {
		sck.debug("applyNextConfig - failed to apply the new config")
		return
	}
	sck.debug("applyNextConfig - apply the new config succeeded")

	sck.put(CurrentConfigKey, next)
	sck.debug("applyNextConfig - succeeded, new=%+v", next)
}

func (sck *ShardCtrler) query(key string) (*shardcfg.ShardConfig, rpc.Tversion) {
	value, version, err := sck.IKVClerk.Get(key)
	if err == rpc.OK {
		return shardcfg.FromString(value), version
	}
	sck.fatalf("query(%s) - failed, err=%s\n", key, err)
	return nil, 0
}

func (sck *ShardCtrler) put(key string, config *shardcfg.ShardConfig) bool {
	sck.debug("put(%s) - config.Num=%d, config=%+v", key, config.Num, config)

	configString := config.String()
	sck.IKVClerk.Put(key, configString, rpc.Tversion(config.Num))
	for {
		currentConfig, currentVersion := sck.query(key)
		if currentVersion > rpc.Tversion(config.Num) {
			if currentConfig.String() == configString {
				sck.debug("put(%s) - succeeded, config.Num=%d, config=%+v", key, config.Num, config)
				return true
			} else {
				sck.debug("put(%s) - failed, config.Num=%d, config=%+v", key, config.Num, config)
				return false
			}
		}
		sck.debug("put(%s) - failed to put new config, try again", key)
		sck.IKVClerk.Put(key, configString, rpc.Tversion(config.Num))
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
			err := shardgrpClerk.InstallShard(shid, nil, 1)
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

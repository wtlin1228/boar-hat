package lock

import (
	"time"

	"6.5840/kvsrv1/rpc"
	kvtest "6.5840/kvtest1"
)

type Lock struct {
	// IKVClerk is a go interface for k/v clerks: the interface hides
	// the specific Clerk type of ck but promises that ck supports
	// Put and Get.  The tester passes the clerk in when calling
	// MakeLock().
	ck kvtest.IKVClerk
	// You may add code here
	id          string
	l           string
	lockVersion rpc.Tversion
}

// The tester calls MakeLock() and passes in a k/v clerk; your code can
// perform a Put or Get by calling lk.ck.Put() or lk.ck.Get().
//
// Use l as the key to store the "lock state" (you would have to decide
// precisely what the lock state is).
func MakeLock(ck kvtest.IKVClerk, l string) *Lock {
	// You may add code here
	id := kvtest.RandValue(8)
	lk := &Lock{ck: ck, id: id, l: l}
	return lk
}

func (lk *Lock) Acquire() {
	// Your code here
	for {
		lockState, version, err := lk.ck.Get(lk.l)
		switch err {
		case rpc.ErrNoKey:
			err := lk.ck.Put(lk.l, lk.id, 0)
			if err == rpc.OK {
				lk.lockVersion = 1
				return
			}
		case rpc.OK:
			if lockState == "unlock" {
				err := lk.ck.Put(lk.l, lk.id, version)
				lk.lockVersion = version + 1
				if err == rpc.OK {
					return
				}
			}
		default:
			// ignore other cases
		}
		time.Sleep(10 * time.Millisecond)
	}
}

func (lk *Lock) Release() {
	// Your code here
	lockState, version, err := lk.ck.Get(lk.l)
	if err == rpc.OK && lockState == lk.id && version == lk.lockVersion {
		lk.ck.Put(lk.l, "unlock", lk.lockVersion)
	}
}

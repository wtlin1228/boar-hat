# Closure

When the Go compiler analyzes this code, it sees that `counter` is referenced not only in the outer function but also inside the nested `inner` function. Because `inner` may outlive `outer`, the compiler _escapes_ `counter` to the heap instead of storing it on the stack.

This escape to the heap is what allows the returned `inner` function to continue accessing and modifying `counter` even after `outer` has returned.

```go
import "fmt"

func outer() func() {
    counter := 0

    return func() {
        if counter >= 3 {
            return
        }
        counter++
        fmt.Printf("counter %d\n", counter)
    }
}

func main() {
    inner := outer()

    for i := 0; i < 10; i++ {
        inner()
    }
}
```

# Patterns

## Publish/subscribe server

```go
type Event struct {
	name string
}

type PubSub interface {
	// Publish publishes the event e to
	// all current subscriptions.
	Publish(e Event)
	// Subscribe registers c to receive future events.
	// All subscribers receive events in the same order,
	// and that order respects program order:
	// if Publish(e1) happens before Publish(e2),
	// subscribers receive e1 before e2.
	Subscribe(c chan<- Event)
	// Cancel cancels the prior subscription of channel c.
	// After any pending already-published events
	// have been sent on c, the server will signal that the
	// subscription is cancelled by closing c.
	Cancel(c chan<- Event)
}
```

from:

```go
type Server struct {
	mu  sync.Mutex
	sub map[chan<- Event]bool
}

func (s *Server) Init() {
	s.sub = make(map[chan<- Event]bool)
}
func (s *Server) Publish(e Event) {
	s.mu.Lock()
	defer s.mu.Unlock()
	for c := range s.sub {
		c <- e
	}
}
func (s *Server) Subscribe(c chan<- Event) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.sub[c] {
		panic("pubsub: already subscribed")
	}
	s.sub[c] = true
}
func (s *Server) Cancel(c chan<- Event) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if !s.sub[c] {
		panic("pubsub: not subscribed")
	}
	close(c)
	delete(s.sub, c)
}
```

to:

```go
import (
	"fmt"
)

type Server struct {
	publish   chan Event
	subscribe chan subReq
	cancel    chan subReq
}

type subReq struct {
	ch chan<- Event
	ok chan bool
}

func (s *Server) Init() {
	s.publish = make(chan Event)
	s.subscribe = make(chan subReq)
	s.cancel = make(chan subReq)
	go s.loop()
}
func (s *Server) Publish(e Event) {
	s.publish <- e
}
func (s *Server) Subscribe(c chan<- Event) {
	r := subReq{ch: c, ok: make(chan bool)}
	s.subscribe <- r
	if !<-r.ok {
		panic("pubsub: already subscribed")
	}
}
func (s *Server) Cancel(c chan<- Event) {
	r := subReq{ch: c, ok: make(chan bool)}
	s.cancel <- r
	if !<-r.ok {
		panic("pubsub: not subscribed")
	}
}

func (s *Server) loop() {
	sub := make(map[chan<- Event]chan<- Event)
	for {
		select {
		case e := <-s.publish:
			for _, h := range sub {
				h <- e
			}
		case r := <-s.subscribe:
			if sub[r.ch] != nil {
				panic("pubsub: already subscribed")
			}
			h := make(chan Event)
			go helper(h, r.ch)
			sub[r.ch] = h
			r.ok <- true
		case r := <-s.cancel:
			if sub[r.ch] == nil {
				panic("pubsub: not subscribed")
			}
			close(sub[r.ch])
			delete(sub, r.ch)
			r.ok <- true
		}
	}
}
func helper(in <-chan Event, out chan<- Event) {
	var q []Event
	for in != nil || len(q) > 0 {
		// Decide whether and what to send.
		var sendOut chan<- Event
		var next Event
		if len(q) > 0 {
			sendOut = out
			next = q[0]
		}

		select {
		case e, ok := <-in:
			if !ok {
				in = nil // stop receiving from in
				break
			}
			q = append(q, e)
		case sendOut <- next:
			q = q[1:]
		}
	}
	close(out)
}
```

## Work scheduler

```go
func Schedule(servers chan string, numTask int, call func(srv string, task int) bool) {
	work := make(chan int, numTask)
	done := make(chan bool)
	exit := make(chan bool)

	runTasks := func(srv string) {
		for task := range work {
			if call(srv, task) {
				done <- true
			} else {
				work <- task
			}
		}
	}

	go func() {
		for {
			select {
			case srv := <-servers:
				go runTasks(srv)
			case <-exit:
				return
			}
		}
	}()

	for task := 0; task < numTask; task++ {
		work <- task
	}

	for i := 0; i < numTask; i++ {
		<-done
	}
	close(work)
	exit <- true
}
```

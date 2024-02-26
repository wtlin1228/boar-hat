use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//   - Mutex + Condvar + VecDeque
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of atomic VecDeque<T>
//  - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//  - Oneshot channels: Any capacity. In practice, only one call to send().
//
// Next:
//  - https://doc.rust-lang.org/src/std/sync/mpsc/mod.rs.html
//  - https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel/src/flavors
//  - https://github.com/zesterer/flume
//  - https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let is_last = inner.senders == 0;
        drop(inner);
        if is_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<(), T> {
        let mut inner = self.shared.inner.lock().unwrap();
        if inner.is_receiver_closed {
            return Err(t);
        }
        inner.queue.push_back(t);
        drop(inner);
        // There is only one receiver, so `notify_one()` will notify the right
        // receiver.
        self.shared.available.notify_one();
        Ok(())
    }
}

// `buffer` is one optimization based on our assumption that "only one receiver".
// When receiver calls `recv()`, the lock isn't required until the `buffer` is
// empty. Then we swap the memory of `buffer` and the `queue`.
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.is_receiver_closed = true;
        drop(inner);
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        // Operating system doesn't guarantee this thread is waken up while
        // queue is not empty. So put a `loop` here to handle that situation
        // when the receiver needs to go to sleep again.
        loop {
            // `queue` could return nothing if the `queue` is empty. And
            // since we want to provide the blocking version of `recv`, which
            // means if there isn't something yet, it waits for something to
            // be in the channel. And that's why the `Condvar` comes into play.
            match inner.queue.pop_front() {
                Some(t) => {
                    if !self.buffer.is_empty() {
                        std::mem::swap(&mut inner.queue, &mut self.buffer);
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    // Operating system will put this thread to sleep and only
                    // wake up when there is some reason to wake up.
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

// `queue` can't be implemented by a `Vec<T>` because then the receiver will
// get the last sent `T`.
struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
    is_receiver_closed: bool,
}

// `Condvar` needs to be outside the `Mutex`. Image you're currently holding
// the `Mutex` then you realize that you need to awake other people up. The
// person you wake up needs to take the lock. But the person can't get the
// `Mutex` since you're currently holding the `Mutex`. This could end up with
// dead lock. So that's why the `Condvar` requires you to give it a `Mutex
// Guard`. You have to prove that you currently hold the lock.
struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::new(),
        senders: 1,
        is_receiver_closed: false,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver {
            shared: Arc::clone(&shared),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        assert_eq!(tx.send(42), Err(42));
    }
}

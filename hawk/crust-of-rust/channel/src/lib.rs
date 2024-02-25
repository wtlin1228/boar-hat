use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        // There is only one receiver, so `notify_one()` will notify the right
        // receiver.
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        // Operating system doesn't guarantee this thread is waken up while
        // queue is not empty. So put a `loop` here to handle that situation
        // when the receiver needs to go to sleep again.
        loop {
            // `queue` could return nothing if the `queue` is empty. And
            // since we want to provide the blocking version of `recv`, which
            // means if there isn't something yet, it waits for something to
            // be in the channel. And that's why the `Condvar` comes into play.
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    // Operating system will put this thread to sleep and only
                    // wake up when there is some reason to wake up.
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

// `queue` can't be implemented by a `Vec<T>` because then the receiver will
// get the last sent `T`.
//
// `Condvar` needs to be outside the `Mutex`. Image you're currently holding
// the `Mutex` then you realize that you need to awake other people up. The
// person you wake up needs to take the lock. But the person can't get the
// `Mutex` since you're currently holding the `Mutex`. This could end up with
// dead lock. So that's why the `Condvar` requires you to give it a `Mutex
// Guard`. You have to prove that you currently hold the lock.
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: Arc::clone(&inner),
        },
        Receiver {
            inner: Arc::clone(&inner),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}

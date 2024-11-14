use std::task::Poll;

#[derive(Default)]
struct Executor {
    tasks: Vec<Task>,
    current_task_idx: Option<usize>,
}

impl Executor {
    fn has_task(&self) -> bool {
        self.tasks.len() > 0
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn poll(&mut self) -> Option<Task> {
        if self.tasks.len() == 0 {
            self.current_task_idx = None;
            return None;
        }
        let i = match self.current_task_idx {
            Some(n) => (n + 1) % self.tasks.len(),
            None => 0,
        };
        let task = self.tasks.get_mut(i).unwrap();
        match task.poll() {
            Poll::Ready(_) => {
                let task = self.tasks.remove(i);
                Some(task)
            }
            Poll::Pending => {
                self.current_task_idx = Some(i);
                None
            }
        }
    }
}

struct Task {
    identifier: String,
    counter: usize,
    yield_every: usize,
    done: bool,
}

impl Task {
    pub fn new(identifier: String, yield_every: usize) -> Self {
        Self {
            identifier,
            counter: 0,
            yield_every,
            done: false,
        }
    }

    fn poll(&mut self) -> Poll<()> {
        loop {
            self.counter += 1;
            println!("task {} count to {}", self.identifier, self.counter);
            if self.counter > 100 {
                self.done = true;
                return Poll::Ready(());
            } else if self.counter % self.yield_every == 0 {
                return Poll::Pending;
            }
        }
    }
}

fn main() {
    let mut executor = Executor::default();
    let task_1 = Task::new("task_1".to_string(), 1);
    let task_5 = Task::new("task_5".to_string(), 5);
    let task_10 = Task::new("task_10".to_string(), 10);

    executor.add_task(task_1);
    executor.add_task(task_5);
    executor.add_task(task_10);

    while executor.has_task() {
        if let Some(task) = executor.poll() {
            println!("complete task {}", task.identifier);
        }
    }
}

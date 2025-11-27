- A process is a single program we're running it has a single address space it can have multiple threads
- Threads within the same process can share memory, use mutexes but not between processes
- Operating system knows about the threads of each process, so when the scheduler try to pick a new thread to run, it can be a thread in the same process or different process.

![illustration](./illustration.png)

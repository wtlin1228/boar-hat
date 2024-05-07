# 6.S081 Operating System Engineering

- Course web site: https://pdos.csail.mit.edu/6.828/2020/
- xv6: https://github.com/mit-pdos/xv6-riscv
- xv6 Book: https://pdos.csail.mit.edu/6.828/2020/xv6/book-riscv-rev1.pdf
- Pointers in C language: C Programming Language, 2nd Edition, CHAPTER 5: Pointers and Arrays

# Operating System Interface

| System call                           | Description                                                              |
| ------------------------------------- | ------------------------------------------------------------------------ |
| int fork()                            | Create a process, return child’s PID.                                    |
| int exit(int status)                  | Terminate the current process; status reported to wait(). No return.     |
| int wait(int \*status)                | Wait for a child to exit; exit status in \*status; returns child PID.    |
| int kill(int pid)                     | Terminate process PID. Returns 0, or -1 for error.                       |
| int getpid()                          | Return the current process’s PID.                                        |
| int sleep(int n)                      | Pause for n clock ticks.                                                 |
| int exec(char *file, char *argv[])    | Load a file and execute it with arguments; only returns if error.        |
| char \*sbrk(int n)                    | Grow process’s memory by n bytes. Returns start of new memory.           |
| int open(char \*file, int flags)      | Open a file; flags indicate read/write; returns an fd (file descriptor). |
| int write(int fd, char \*buf, int n)  | Write n bytes from buf to file descriptor fd; returns n.                 |
| int read(int fd, char \*buf, int n)   | Read n bytes into buf; returns number read; or 0 if end of file.         |
| int close(int fd)                     | Release open file fd.                                                    |
| int dup(int fd)                       | Return a new file descriptor referring to the same file as fd.           |
| int pipe(int p[])                     | Create a pipe, put read/write file descriptors in p[0] and p[1].         |
| int chdir(char \*dir)                 | Change the current directory.                                            |
| int mkdir(char \*dir)                 | Create a new directory.                                                  |
| int mknod(char \*file, int, int)      | Create a device file.                                                    |
| int fstat(int fd, struct stat \*st)   | Place info about an open file into \*st.                                 |
| int stat(char *file, struct stat *st) | Place info about a named file into \*st.                                 |
| int link(char *file1, char *file2)    | Create another name (file2) for the file file1.                          |
| int unlink(char \*file)               | Remove a file.                                                           |

### `fork()` & `wait()`

Fork returns zero in the child process and returns child's PID in the parent process. The child process has the same memory contents as the parent process. `shell` runs programs on behalf of users by combining the system calls:

```c
int main(void)
{
    // Read and run input commands.
    while (getcmd(buf, sizeof(buf)) >= 0)
    {
        if (fork1() == 0)
        {
            runcmd(parsecmd(buf));
        }
        wait(0);
    }
    exit(0);
}
```

### `read()` & `write()`

Each process has it's own file descriptor table. And 0, 1, 2 is conventionally used for `console`.

| file descriptor index | Underlining I/O Object |
| --------------------- | ---------------------- |
| 0                     | standard input         |
| 1                     | standard output        |
| 2                     | standard error         |

```c
int cat()
{
    char buf[512];
    int n;
    for (;;)
    {
        n = read(0, buf, sizeof buf);

        if (n == 0)
        {
            break;
        }

        if (n < 0)
        {
            fprintf(2, "read error\n");
            exit(1);
        }

        if (write(1, buf, n) != n)
        {
            fprintf(2, "write error\n");
            exit(1);
        }
    }
}
```

### Share offset between file descriptors

`fork()` and `dup()` calls can share the offset between file descriptors.

```c
if (fork() == 0)
{
    write(1, "hello ", 6);
    exit(0);
}
else
{
    wait(0);
    write(1, "world\n", 6);
}
```

```c
fd = dup(1);
write(1, "hello ", 6);
write(fd, "world\n", 6);
```

At the end of both fragments, the file attached to file descriptor 1 will contain the data `hello world`.

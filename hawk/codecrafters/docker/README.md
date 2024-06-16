[![progress-banner](https://backend.codecrafters.io/progress/docker/00c70c98-856c-4df1-8b7f-0e8cbddf0b10)](https://app.codecrafters.io/users/wtlin1228?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Docker" Challenge](https://codecrafters.io/challenges/docker).

In this challenge, you'll build a program that can pull an image from
[Docker Hub](https://hub.docker.com/) and execute commands in it. Along the way,
we'll learn about [chroot](https://en.wikipedia.org/wiki/Chroot),
[kernel namespaces](https://en.wikipedia.org/wiki/Linux_namespaces), the
[docker registry API](https://docs.docker.com/registry/spec/api/) and much more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your Docker implementation is `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master
```

That's all!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

You'll use linux-specific syscalls in this challenge. so we'll run your code
_inside_ a Docker container.

Please ensure you have [Docker installed](https://docs.docker.com/get-docker/)
locally.

Next, add a [shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mydocker='docker build -t mydocker . && docker run --cap-add="SYS_ADMIN" mydocker'
```

(The `--cap-add="SYS_ADMIN"` flag is required to create
[PID Namespaces](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html))

You can now execute your program like this:

```sh
mydocker run alpine:latest /usr/local/bin/docker-explorer echo hey
```

This command compiles your Rust project, so it might be slow the first time you
run it. Subsequent runs will be fast.

# Resource

- [Container Runtime in Rust — Part 0](https://itnext.io/container-runtime-in-rust-part-0-7af709415cda)
- [Container Runtime in Rust — Part I](https://itnext.io/container-runtime-in-rust-part-i-7bd9a434c50a)
- [Container Runtime in Rust — Part II](https://itnext.io/container-runtime-in-rust-part-ii-9c88e99d8cbc)
- [PURA - Lightweight & OCI-compliant container runtime](https://github.com/penumbra23/pura)
- [youki: A container runtime in Rust](https://github.com/containers/youki)
- [Container security fundamentals](https://www.youtube.com/playlist?list=PLdh-RwQzDsaNWBex2I09OFLCph7l_KnQE)

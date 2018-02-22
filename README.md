# Hello, Rust!

# logs

# create project

```
$ dcom run rust cargo new guessing_game --bin
$ ls guessing_game/*
guessing_game/Cargo.toml

guessing_game/src:
main.rs

$ mv guessing_game/* .

# build
$ dcom run rust cargo build
    Compiling guessing_game v0.1.0 (file:///code)
    Finished dev [unoptimized + debuginfo] target(s) in 2.21 secs

# build and run
$ dcom run rust cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/guessing_game`
Hello, world!
```

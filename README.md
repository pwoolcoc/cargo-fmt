# Cargo-fmt

[![Build Status](https://travis-ci.org/pwoolcoc/cargo-fmt.svg)](https://travis-ci.org/pwoolcoc/cargo-fmt)

Allows you to call [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt)
through `cargo`. Useful when using
[`cargo-do`](https://github.com/pwoolcoc/cargo-do), as it allows you to do
`cargo do fmt, build`.

Right now there is a restriction that you have to run it from the top
level of your project, it won't search up the directory tree like cargo
will.

There is also no way, right now, to pass arguments through to `rustfmt`.

## Install

```bash
$ git clone https://github.com/pwoolcoc/cargo-fmt
$ cd cargo-fmt
$ cargo build --release
$ cp target/release/cargo-fmt /usr/local/bin/
```

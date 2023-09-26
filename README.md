<div align="center">
  <h1>Eyra</h1>

  <p>
    <strong>Rust programs written entirely in Rust</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/eyra/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/eyra/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/eyra"><img src="https://img.shields.io/crates/v/eyra.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/eyra"><img src="https://docs.rs/eyra/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

Eyra is a package that supports building Rust programs implemented entirely
in Rust.

It uses [Origin] for program and thread startup and shutdown, and [c-gull] for
ABI-compatible libc function implementations.

[Origin]: https://github.com/sunfishcode/origin#readme
[c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull#readme

## Quick start

Check out [this hello world example].

[this hello world example]: https://github.com/sunfishcode/eyra/tree/main/example-crates/hello-world/

## In detail

Eyra needs three things. First, a Cargo.toml dependency:

```toml
[dependencies]
eyra = "<current-version>"
```

The next step is to add an `extern crate`:

```rust
extern crate eyra;

fn main() {
    println!("Hello, world!");
}
```

This tells Rust that Eyra is actually used and the libraries should actually
be linked in.

And finally, a build.rs file to add `-nostartfiles` to the link flags to
disable the host startup code, so that Eyra can provide its own. build.rs:

```rust,no_run
fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
}
```

With these three steps, this crate prints "Hello, world!". And under the
covers, it uses [Origin] to start and stop the program, [c-ward] to handle
libc calls from `std`, and [rustix] to do the printing, so it's completely
implemented in Rust.

## Optional logging

Eyra has a `log` feature to enable Rust `log` tracing of program and thread
startup and shutdown, and an `env_logger` feature to install `env_logger`
as the logger, which can be enabled in Cargo.toml:

```toml
[dependencies]
eyra = { version = "<current-version>", features = ["log", "env_logger"] }
```

With this, and setting the `RUST_LOG` environment variable to "trace", the
hello world program output like this:

```console
[TRACE origin::program] Program started
[TRACE origin::thread] Main Thread[Pid(51383)] initialized
[TRACE origin::program] Calling `.init_array`-registered function `0x55e86306bb80(1, 0x7ffd0f76aad8, 0x7ffd0f76aae8)`
[TRACE origin::program] Calling `origin_main(1, 0x7ffd0f76aad8, 0x7ffd0f76aae8)`
Hello, world!
[TRACE origin::program] `origin_main` returned `0`
[TRACE origin::thread] Thread[51383] calling `at_thread_exit`-registered function
[TRACE origin::thread] Thread[51383] calling `at_thread_exit`-registered function
[TRACE origin::program] Program exiting
```

## Background

Eyra is similar to [Mustang] and uses the same underlying code, but instead
of using a custom target and -Z build-std, Eyra just needs users to add
`-nostartfiles` to their link line, such as via build.rs in the example.

Like Mustang, Eyra currently runs on Rust Nightly on Linux on x86-64, x86,
aarch64, and riscv64. It aims to support all Linux versions
[supported by Rust], though at this time it's only tested on relatively recent
versions. It's complete enough to run:
 - [ripgrep](https://github.com/sunfishcode/ripgrep/tree/eyra)
 - [coreutils](https://github.com/sunfishcode/coreutils/tree/eyra),
   including the "unix" feature set
 - [async-std](https://github.com/sunfishcode/tide/tree/eyra)
 - [tokio](https://github.com/sunfishcode/tokio/tree/eyra)
 - [bat](https://github.com/sunfishcode/bat/tree/eyra), including git
   support with libgit2
 - [cargo-watch](https://github.com/sunfishcode/cargo-watch/tree/eyra)

Eyra isn't about making anything safer, for the foreseeable future. The major
libc implementations are extraordinarily well tested and mature. Eyra for its
part is experimental and contains lots of `unsafe`.

## Design philosophy

Eyra and the libraries it uses have some design goals.

### Normal Rust, all the way down

Sometimes in libc implementation code, there's a temptation to say "it's ok
if some things are technically Undefined Behavior, because this is Low Level
Code and We Know What We're Doing.

Origin, c-scape, c-gull, rustix, and the others strive to resist this
temptation, and follow the Rust rules, including strict provenance, I/O safety,
and all the rest, all the way down to the syscalls.

It's just normal Rust code, as far down as we can go in userspacce, and when we
eventually do have to switch to inline asm, we do as little of it as we can.

Currently there is only one known place where this goal is not achieved. In a
"static PIE" executable (eg. built with
`RUSTFLAGS="-C target-feature=+crt-static")`, the dynamic linker isn't used,
so the executable has to handle all its relocations itself. Origin's code for
doing this is currently disabled by default, and can be enabled with the
"experimental-relocate" cargo feature.

### C compatibility as a layer on top of Rust, not vice versa

Eyra is built on a collection of Rust crates with idiomatic Rust APIs, and two
crates, c-scape and c-gull, which are relatively thin layers on top that
implement the libc-compatible C ABI.

It's sometimes more work to write the code as separate layers like this, but
it has the advantage of clearly separating out the `unsafe` associated with
things like C pointers and strings in libc APIs from the essential `unsafe`
needed to implement things like system calls, thread primitives, and other
features. And it means that Rust programs that don't want to go through the C
compatibility layer can use the underlying crates directly.

[Mustang]: https://github.com/sunfishcode/mustang#readme
[Origin]: https://github.com/sunfishcode/origin#readme
[c-ward]: https://github.com/sunfishcode/c-ward#readme
[rustix]: https://github.com/bytecodealliance/rustix#readme
[supported by Rust]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

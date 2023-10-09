This crate demonstrates the use of Eyra with a smaller binary size!

This is the same as the [hello-world] example, but enables some options
described in [min-sized-rust] to reduce the size of the final binary.

It uses the [workaround to support -Zbuild-std], and can be built with
a command like this:

```console
$ RUSTFLAGS="-Zlocation-detail=none -C relocation-model=static -Ctarget-feature=+crt-static" cargo +nightly run -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release
```

This applies all the techniques described on the [min-sized-rust] page
before [Remove `core::fmt` with `#![no_main]` and Careful Usage of `libstd`].

As of this writing, using all these same optimizations without Eyra, using
`x86_64-unknown-linux-musl` (which produces smaller statically-linked binaries
than `x86_64-unknown-linux-gnu`), compiles to 50776 bytes, while this Eyra
example currently compiles to 37912 bytes.

If you're interested in going further down the `#![no_main]`/`#![no_std]`
path, consider [using Origin directly] which can get down to 408 bytes. Or,
consider using [Origin Studio] if you want to go there but still have
`println!`.

[hello-world]: https://github.com/sunfishcode/eyra/tree/main/example-crates/hello-world/
[min-sized-rust]: https://github.com/johnthagen/min-sized-rust
[workaround to support -Zbuild-std]: https://github.com/sunfishcode/eyra/blob/main/README.md#compatibility-with--zbuild-std
[Remove `core::fmt` with `#![no_main]` and Careful Usage of `libstd`]: https://github.com/johnthagen/min-sized-rust#remove-corefmt-with-no_main-and-careful-usage-of-libstd
[using Origin directly]: https://github.com/sunfishcode/origin/tree/main/example-crates/tiny
[Origin Studio]: https://github.com/sunfishcode/origin-studio

This crate demonstrates the use of Eyra as an optional dependency.

By default, it looks like a normal hello world program. When compiled
with `--feature=eyra`, it's a hello world program that uses Eyra.

And when compiled with `--features=eyra,eyra/log,eyra/env_logger`, it's
a hello world program that uses Eyra and can log program startup and
shutdown:

```console
$ RUST_LOG=trace cargo +nightly run --quiet --features=eyra,eyra/log,eyra/env_logger
[TRACE origin::program] Program started
[TRACE origin::thread] Main Thread[Pid(91601)] initialized
[TRACE origin::program] Calling `.init_array`-registered function `0x559006f43500(1, 0x7ffd7e5bacd8, 0x7ffd7e5bace8)`
[TRACE origin::program] Calling `origin_main(1, 0x7ffd7e5bacd8, 0x7ffd7e5bace8)`
Hello, world!
[TRACE origin::program] `origin_main` returned `0`
[TRACE origin::thread] Thread[91601] calling `at_thread_exit`-registered function
[TRACE origin::thread] Thread[91601] calling `at_thread_exit`-registered function
[TRACE origin::program] Program exiting
```

//! Run the programs in the `example-crates` directory and compare their
//! outputs with expected outputs.

#![feature(cfg_target_abi)]

extern crate eyra;

fn test_crate(
    name: &str,
    args: &[&str],
    envs: &[(&str, &str)],
    stdout: &'static str,
    stderr: &'static str,
    code: Option<i32>,
) {
    use assert_cmd::Command;

    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";
    #[cfg(target_arch = "riscv64")]
    let arch = "riscv64gc";
    #[cfg(target_arch = "x86")]
    let arch = "i686";
    #[cfg(target_arch = "arm")]
    let arch = "armv5te";
    #[cfg(target_env = "gnueabi")]
    let env = "gnueabi";
    #[cfg(all(target_env = "gnu", target_abi = "eabi"))]
    let env = "gnueabi";
    #[cfg(all(target_env = "gnu", not(target_abi = "eabi")))]
    let env = "gnu";

    let mut command = Command::new("cargo");
    command.arg("run").arg("--quiet");
    command.arg(&format!("--target={}-unknown-linux-{}", arch, env));
    command.args(args);

    // Special-case "eyra-panic-example" to disable "RUST_BACKTRACE", so that
    // the stderr message is reproducible.
    if name == "eyra-panic-example" {
        command.env_remove("RUST_BACKTRACE");
    }

    command.envs(envs.iter().cloned());
    command.current_dir(format!("example-crates/{}", name));
    let assert = command.assert();
    let assert = assert.stdout(stdout).stderr(stderr);
    if let Some(code) = code {
        assert.code(code);
    } else {
        assert.success();
    }
}

#[test]
fn example_crate_hello_world() {
    test_crate("hello-world", &[], &[], "Hello, world!\n", "", None);
}

#[test]
fn example_crate_eyra_libc_example() {
    test_crate(
        "eyra-libc-example",
        &[],
        &[],
        "Hello world using Rust `println!`!\nHello world using libc `printf`!\n",
        "",
        None,
    );
}

#[test]
fn example_crate_eyra_panic_example() {
    test_crate(
        "eyra-panic-example",
        &[],
        &[],
        "",
        "thread 'main' panicked at src/main.rs:4:5:\nUh oh!\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n",
        Some(101)
    );
}

#[test]
fn example_crate_eyra_optional_example() {
    // Test the crate in non-Eyra mode.
    test_crate(
        "eyra-optional-example",
        &[],
        &[],
        "Hello, world!\n",
        "",
        None,
    );

    // Test the crate in Eyra mode.
    test_crate(
        "eyra-optional-example",
        &["--features=eyra"],
        &[],
        "Hello, world!\n",
        "",
        None,
    );
}

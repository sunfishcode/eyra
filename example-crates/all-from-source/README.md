This example demonstrates building a program completely from source, without
using any prebuilt libraries, if built with `-Zbuild-std`.

For example, use `cargo rustc` to build the LLVM IR file and test that it has
no external function declarations (other than LLVM intrinsics).

```
$ cargo +nightly rustc -Zbuild-std --release --target=x86_64-unknown-linux-gnu  -- --emit=llvm-ir
[...]
$ grep ^declare ./target/x86_64-unknown-linux-gnu/release/deps/all_from_source-*.ll | grep -v '@llvm'
$
``` 

# How to build & run

From folder root:

1. cargo build
2. clang src/c/main.c -Wl target/debug/libc_with_rust.a -o target/debug/build/main
3. ./target/debug/build/main

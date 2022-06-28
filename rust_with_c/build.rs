use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/c/cool.c");
    println!("cargo:rerun-if-changed=src/c/cool.h");

    // Call clang to build our cool c lib
    //  -c Only run preprocess, compile, and assemble steps
    //  -o <file>               Write output to <file>
    Command::new("clang")
        .args(["-c", "src/c/cool.c", "-o", "src/c/cool.o"])
        .status()
        .unwrap();

    // Call ar to create archive
    // r[ab][f][u] Replace existing or insert new file(s) into the archive
    Command::new("ar")
        .args(["r", "src/c/libcool.a", "src/c/cool.o"])
        .status()
        .unwrap();

    println!("cargo:rustc-link-search={}", "src/c");
}

use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=generators/GenRandomResults.java");
    println!("cargo:rerun-if-changed=external");

    Command::new("javac")
        .args(["generators/GenRandomResults.java"])
        .output()
        .expect("could not compile generator");

    Command::new("java")
        .args(["-cp", "generators", "GenRandomResults"])
        .output()
        .expect("could not run generator");

    fs::remove_file("generators/GenRandomResults.class").unwrap();

    cc::Build::new()
        .include("external")
        .define("_IEEE_LIBM", None)
        .define("__STDC__", None)
        .define("__LITTLE_ENDIAN", None)
        .file("external\\fdlibm\\e_sqrt.c")
        .file("external\\fdlibm\\e_log.c")
        .file("external\\fdlibm\\w_sqrt.c")
        .file("external\\fdlibm\\w_log.c")
        .compile("fdlibm");
}

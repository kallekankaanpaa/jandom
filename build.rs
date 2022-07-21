use std::{env, process};

fn main() {
    println!("cargo:rerun-if-changed=generators/GenRandomResults.java");
    println!("cargo:rerun-if-changed=external");
    let out_dir = env::var("OUT_DIR").unwrap();

    process::Command::new("javac")
        .args(["generators/GenRandomResults.java", "-d", &out_dir])
        .output()
        .expect("could not compile generator");

    process::Command::new("java")
        .args(["-cp", &out_dir, "GenRandomResults"])
        .output()
        .expect("could not run generator");

    cc::Build::new()
        .include("external")
        .define("_IEEE_LIBM", None)
        .define("__LITTLE_ENDIAN", None)
        .file("external\\fdlibm\\e_sqrt.c")
        .file("external\\fdlibm\\e_log.c")
        .compile("fdlibm");
}

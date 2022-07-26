use std::{env, process};

fn main() {
    if let Err(_) = env::var("DOCS_RS") {
        let out_dir = env::var("OUT_DIR").unwrap();
        println!("cargo:rerun-if-changed=generators/GenRandomResults.java");

        process::Command::new("javac")
            .args(["generators/GenRandomResults.java", "-d", &out_dir])
            .output()
            .expect("could not compile generator");

        process::Command::new("java")
            .args(["-cp", &out_dir, "GenRandomResults"])
            .output()
            .expect("could not run generator");
    }

    println!("cargo:rerun-if-changed=external");
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

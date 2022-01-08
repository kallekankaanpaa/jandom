use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=generators/GenRandomResults.java");
    println!("cargo:rerun-if-changed=external");

    if cfg!(target_os = "windows") {
        Command::new("javac")
            .args(["generators\\GenRandomResults.java"])
            .output()
            .expect("could not compile generator");
    } else {
        Command::new("javac")
            .args(["generators/GenRandomResults.java"])
            .output()
            .expect("could not compile generator");
    }

    Command::new("java")
        .args(["-cp", "generators", "GenRandomResults"])
        .output()
        .expect("could not run generator");

    cc::Build::new()
        .include("external")
        .define("_IEEE_LIBM", None)
        .define("__STDC__", None)
        .define("__LITTLE_ENDIAN", None)
        .file("external\\fdlibm\\e_sqrt.c")
        .file("external\\fdlibm\\e_log.c")
        .compile("fdlibm");
}

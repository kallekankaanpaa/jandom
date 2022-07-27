use std::{env, process};

fn main() {
    if env::var("DOCS_RS").is_err() {
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
}

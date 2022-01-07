use std::process::Command;

fn main() {
    //println!("cargo:rerun-if-changed=generators/GenRandomResults.java");

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
}

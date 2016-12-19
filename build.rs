use std::process::{Command, Stdio};

fn main() {

    let ret = Command::new("make")
        .arg("-C")
        .arg("libpg_query_lib")
        .arg("build")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(ret.success());

    println!("cargo:rustc-flags=-L libpg_query_lib -l static=pg_query");
}

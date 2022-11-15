use std::{time, env, ffi::OsString, fs, io::prelude::*, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: String = args[0].clone().into();
    let input_file: String = args[1].clone().into();
    let file = fs::File::open(input_file).expect("file not found");

    process::Command::new("cargo")
        .args(["build", "--bin", &target]);

    

    let start_time = time::SystemTime::now();

    let end_time = time::SystemTime::now();
}

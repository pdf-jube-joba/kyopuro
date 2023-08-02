use std::{time, env, fs, process::{self, Command}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: String = args[0].clone().into();
    let input_file: String = args[1].clone().into();
    let file = fs::File::open(input_file).expect("file not found");

    let build_out = process::Command::new("cargo")
        .args(["build", "--bin", &target])
        .output();

    if let Ok(out) = build_out {
        println!("build success with {out:?}");
    } else {
        println!("build fail");
        process::exit(0);
    }

    let start_time = time::SystemTime::now();

    let output = process::Command::new("cargo")
        .args(["run", "--bin", &target])
        .output();

    let end_time = time::SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    println!("{duration:?}")
}

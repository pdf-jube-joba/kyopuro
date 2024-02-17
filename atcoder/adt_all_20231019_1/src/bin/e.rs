use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        a: [Usize1; n],
    }
    println!("{}", log_parent(a).into_iter().join("\n"))
}

fn log_parent(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut generation: Vec<usize> = vec![0];
    for ai in a {
        let gen = generation[ai];
        generation.push(gen + 1);
        generation.push(gen + 1);
    }
    generation
}

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", approximate_equalization2(a))
}

fn approximate_equalization2(mut a: Vec<usize>) -> usize {
    let n = a.len();
    let ave_fl = a.iter().sum::<usize>() as f64 / n as f64;
    let mut le = 0;
    let mut gt = 0;
    for ai in a {
    }
    todo!()
}

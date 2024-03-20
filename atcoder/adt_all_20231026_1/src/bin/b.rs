use std::cmp::min;

fn main() {
    proconio::input! {
        n: usize, p: usize, q: usize,
        d: [usize; n],
    }
    let m = min(p, q + d.into_iter().min().unwrap());
    println!("{}", m)
}

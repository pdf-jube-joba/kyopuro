use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    println!("{}", re(n, a).into_iter().map(|i| i + 1).join(" "))
}

fn re(n: usize, a: Vec<usize>) -> Vec<usize> {
    let m = a.len();
    let mut v = (0..n).map(|i| (i, false)).collect_vec();
    for ai in a {
        v[ai].1 = true;
    }
    v.split_inclusive(|b| !b.1)
        .flat_map(|v| v.iter().rev().map(|v| v.0))
        .collect_vec()
}

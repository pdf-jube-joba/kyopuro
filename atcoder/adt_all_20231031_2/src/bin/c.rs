use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        c: [Chars; h],
    }
    let x = (0..w).map(|j| (0..h).filter(|&i| c[i][j]  == '#').count()).collect_vec();
    println!("{}", x.into_iter().join(" "))
}

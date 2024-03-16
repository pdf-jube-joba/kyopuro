use itertools::{iproduct, Itertools};
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let tokens = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == 'o')
        .collect_vec();
    let [token1, token2,..] = tokens[..] else {
        unreachable!()
    };
    let dist = token1.0.abs_diff(token2.0) + token1.1.abs_diff(token2.1);
    println!("{dist}");
}

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::marker::{Chars, Usize1};

fn main() {
    proconio::input! {
        n: usize, q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q]
    }
    for ans in count(s, lr) {
        println!("{}", ans);
    }
}

fn count(s: Vec<char>, ls: Vec<(usize, usize)>) -> Vec<usize> {
    let n = s.len();
    // sum[i in 0..n] = #{ k in 0..i | s[i] == s[i+1] }
    let sum: Vec<usize> = std::iter::once(0)
        .chain(
            (0..n - 1)
                .map(|i| if s[i] == s[i + 1] { 1 } else { 0 })
                .cumsum(),
        )
        .collect_vec();
    ls.into_iter()
        .map(|(l, r)| {
            // #{ k in 0..r | s[i] == s[i+1]} - #{ k in 0..l | s[i] == s[i+1]} = #{ k in l..r | s[i] == s[i+1] }
            sum[r] - sum[l]
        })
        .collect_vec()
}

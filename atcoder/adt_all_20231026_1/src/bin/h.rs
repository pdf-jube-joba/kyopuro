use std::collections::VecDeque;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};

fn main() {
    proconio::input! {
        n: usize,
        a: (Usize1, Usize1), b: (Usize1, Usize1),
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    println!(
        "{}",
        if let Some(step) = bishop2(a, b, s) {
            step as isize
        } else {
            -1
        }
    )
}

fn bishop2(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();
    let mut dist = vec![vec![std::usize::MAX; n]; n];
    dist[a.0][a.1] = 0;
    todo!()
}

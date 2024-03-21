use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};

const DIRECTION: [(isize, isize); 4] = [(-1, 1), (1, 1), (-1, -1), (1, -1)];

fn main() {
    proconio::input! {
        n: usize,
        a: (Usize1, Usize1), b: (Usize1, Usize1),
        s: [Chars; n],
    }
    let s = conv(s);
    println!(
        "{}",
        if let Some(step) = bishop2(a, b, s) {
            step as isize
        } else {
            -1
        }
    )
}

fn conv(a: Vec<Vec<char>>) -> Vec<Vec<bool>> {
    a.into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn bishop2(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();
    let moveto = |from: (usize, usize), diff: (isize, isize)| -> Option<(usize, usize)> {
        let t = (from.0 as isize + diff.0, from.1 as isize + diff.1);
        if 0 <= t.0 && t.0 < n as isize && 0 <= t.1 && t.1 < n as isize {
            Some((t.0 as usize, t.1 as usize))
        } else {
            None
        }
    };

    let mut dist = vec![vec![std::usize::MAX; n]; n];
    dist[a.0][a.1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(a);

    while let Some(v) = queue.pop_front() {
        for d in DIRECTION {
            let mut count = 0;
            loop {
                count += 1;
                let Some(n) = moveto((v.0, v.1), (d.0 * count, d.1 * count)) else {
                    break;
                };

                if s[n.0][n.1] {
                    break;
                }

                if dist[n.0][n.1] == dist[v.0][v.1] + 1 {
                    continue;
                }

                if dist[n.0][n.1] != std::usize::MAX {
                    break;
                }

                dist[n.0][n.1] = dist[v.0][v.1] + 1;
                queue.push_back(n);
            }
        }
    }

    if dist[b.0][b.1] == std::usize::MAX {
        None
    } else {
        Some(dist[b.0][b.1])
    }
}

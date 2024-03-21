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

fn move_to(n: usize, from: (usize, usize), diff: (isize, isize)) -> Option<(usize, usize)> {
    let t = (from.0 as isize + diff.0, from.1 as isize + diff.1);
    if 0 <= t.0 && t.0 < n as isize && 0 <= t.1 && t.1 < n as isize {
        Some((t.0 as usize, t.1 as usize))
    } else {
        None
    }
}

fn bishop2(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();

    let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; n];

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((a, 0));

    while let Some((next, step)) = queue.pop_front() {
        if dist[next.0][next.1] != std::usize::MAX {
            continue;
        }
        dist[next.0][next.1] = step;
        for dir in DIRECTION {
            for d in 0..n {
                let Some(t) = move_to(n, next, (d as isize * dir.0, d as isize * dir.1)) else {
                    break;
                };
                if s[t.0][t.1] {
                    break;
                }

                queue.push_back((t, step + 1));
            }
        }
    }

    if dist[b.0][b.1] == std::usize::MAX {
        None
    } else {
        Some(dist[b.0][b.1])
    }
}

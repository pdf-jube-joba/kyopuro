use std::collections::VecDeque;

use itertools::Itertools;
use num_traits::Pow;

fn main() {
    proconio::input! {
        n: usize, d: usize,
        xy: [(isize, isize); n],
    }
    let ans = virus(d, xy);
    for ans in ans {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

fn virus(d: usize, xy: Vec<(isize, isize)>) -> Vec<bool> {
    let n = xy.len();
    let ds = (0..n)
        .map(|i| {
            (0..n)
                .filter(|&j| i != j && in_abs_d(xy[i], xy[j], d))
                .collect_vec()
        })
        .collect_vec();
    let mut is_infect = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    while let Some(n) = queue.pop_front() {
        if is_infect[n] {
            continue;
        }
        is_infect[n] = true;
        for &nei in &ds[n] {
            queue.push_back(nei)
        }
    }
    is_infect
}

fn in_abs_d((x0, y0): (isize, isize), (x1, y1): (isize, isize), d: usize) -> bool {
    x0.abs_diff(x1).pow(2) + y0.abs_diff(y1).pow(2) <= d.pow(2)
}

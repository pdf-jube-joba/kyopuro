use std::collections::VecDeque;

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut ta = vec![];
    for _ in 0..n {
        proconio::input! {
            t: usize, k: usize, a: [Usize1; k],
        }
        ta.push((t, a));
    }
    println!("{}", martial_artist(ta))
}

fn martial_artist(ta: Vec<(usize, Vec<usize>)>) -> usize {
    let n = ta.len();
    let mut learn = vec![false; n];
    let mut t = 0;
    let mut queue = VecDeque::new();
    queue.push_back(n-1);
    while let Some(p) = queue.pop_front() {
        if learn[p] {
            continue;
        }
        learn[p] = true;
        t += ta[p].0;
        for &a in &ta[p].1 {
            queue.push_back(a);
        }
    }
    t
}

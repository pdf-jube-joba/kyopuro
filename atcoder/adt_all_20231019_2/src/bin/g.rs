use std::collections::VecDeque;

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
    }
    let mut a = vec![];
    for _ in 0..m {
        proconio::input! {
            k: usize,
            mut ai: [Usize1; k],
        }
        ai.reverse();
        a.push(ai);
    }
    println!("{}", if pop_2(n, a) { "Yes" } else { "No" })
}

fn pop_2(n: usize, mut a: Vec<Vec<usize>>) -> bool {
    let m = a.len();
    // seen_count[c] = [a0,..,at] <=> last of a[ai] == c
    let mut seen_count: Vec<Vec<usize>> = vec![vec![]; n];
    // c in pop_queue <=> seen_count[c] = [a0, a1]
    let mut pop_queue: VecDeque<usize> = VecDeque::new();

    for (i, ai) in a.iter().enumerate() {
        if let Some(col) = ai.last() {
            seen_count[*col].push(i);
        }
    }
    for i in 0..n {
        if seen_count[i].len() == 2 {
            pop_queue.push_back(i);
        };
    }

    while let Some(col) = pop_queue.pop_front() {
        let [ai0, ai1] = seen_count[col][..] else {
            unreachable!()
        };
        let (c0, c1) = (a[ai0].pop().unwrap(), a[ai1].pop().unwrap());
        debug_assert!(c0 == col && c1 == col);
        let mut n_col = vec![];
        if let Some(&col) = a[ai0].last() {
            n_col.push(col);
            seen_count[col].push(ai0);
        }
        if let Some(&col) = a[ai1].last() {
            n_col.push(col);
            seen_count[col].push(ai1);
        }
        n_col.dedup();
        for col in n_col {
            if seen_count[col].len() == 2 {
                pop_queue.push_back(col);
            }
        }
    }

    a.into_iter().all(|ai| ai.is_empty())
}

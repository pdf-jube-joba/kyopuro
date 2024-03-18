use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    proconio::input! {
        t: usize,
    }
    for _ in 0..t {
        proconio::input! {
            n: usize,
            lr: [(usize, usize); n],
        }
        let ans = packing_under_range_regulations(lr);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

fn packing_under_range_regulations(mut lr: Vec<(usize, usize)>) -> bool {
    lr.sort();
    let n = lr.len();
    let mut i = 0;
    let mut list: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut l_index = 0;
    'outer: loop {
        eprintln!("o:{i}");
        if let Some(Reverse(rj)) = list.pop() {
            if rj <= i {
                return false;
            }
            i += 1;
        } else {
            i = lr[l_index].0;
        }
        eprintln!("o?{i}");

        eprintln!("s: {}", l_index);
        while lr[l_index].0 == i {
            list.push(Reverse(lr[l_index].1));
            l_index += 1;
            if l_index == n - 1 {
                break 'outer;
            }
        }
        eprintln!("e: {}", l_index);
    }
    true
}

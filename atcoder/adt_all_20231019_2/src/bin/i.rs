use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    proconio::input! {
        n: usize, l: usize,
        a: [usize; n],
    };
    println!("{}", min_cost(l, a));
}

fn min_cost(l: usize, mut a: Vec<usize>) -> usize {
    let rem = l - a.iter().sum::<usize>();
    if rem != 0 {
        a.push(rem);
    }
    let mut min_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for ai in a {
        min_heap.push(Reverse(ai));
    }
    let mut cost = 0;
    while let (Some(Reverse(s0)), Some(Reverse(s1))) = (min_heap.pop(), min_heap.pop()) {
        cost += s0 + s1;
        min_heap.push(Reverse(s0 + s1));
    }
    cost
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", min_swap(s, "atcoder".chars().collect_vec()))
}

// goal shuould be reachable
fn min_swap(start: Vec<char>, goal: Vec<char>) -> usize {
    type Edge = Vec<char>;
    let len = start.len();

    let mut edge_n: HashMap<Edge, usize> = HashMap::new();

    let mut queue: BinaryHeap<(Reverse<usize>, Edge)> = BinaryHeap::new();
    queue.push((Reverse(0), start));

    while let Some((Reverse(dep), next)) = queue.pop() {
        if next == goal {
            return dep;
        }

        if edge_n.contains_key(&next) {
            continue;
        }

        edge_n.insert(next.clone(), dep);

        for i in 0..len - 1 {
            let mut new_s = next.clone();
            new_s.swap(i, i + 1);
            if edge_n.get(&new_s).filter(|dist| **dist < dep + 1).is_none() {
                queue.push((Reverse(dep + 1), new_s));
            }
        }
    }

    unreachable!()
}

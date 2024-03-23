use std::{cmp::Reverse, collections::HashSet};
use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        st: [(Chars, usize); n],
    }
    println!("{}", poem_online_judge(st) + 1);
}

fn poem_online_judge(st: Vec<(Vec<char>, usize)>) -> usize {
    let mut num_same: HashSet<Vec<char>> = HashSet::new();
    let mut originals: Vec<usize> = vec![];
    for (i, (s, t)) in st.iter().enumerate() {
        if !num_same.contains(s) {
            num_same.insert(s.clone());
            originals.push(i);
        }
    }
    originals.sort_by_key(|i| Reverse(st[*i].1));
    originals[0]
}

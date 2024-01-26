use std::cmp::Reverse;

use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: [Chars; n],
    }
    let mut count_vec = s
        .iter()
        .map(|line| line.iter().filter(|char| **char == 'o').count())
        .enumerate()
        .collect_vec();

    count_vec.sort_by_key(|(i, count)| (Reverse(*count), *i));
    let str = count_vec
        .into_iter()
        .map(|(i, _)| (i + 1).to_string())
        .join(" ");
    println!("{}", str);
}

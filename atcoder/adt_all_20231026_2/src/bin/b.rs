use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        mut s: Chars
    }
    for i in 0..(s.len()/2) {
        s.swap(2 * i, 2 * i + 1);
    }
    println!("{}", s.into_iter().join(""))
}

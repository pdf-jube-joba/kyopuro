use itertools::Itertools;
use proconio::marker::{Chars, Usize1};

fn main() {
    proconio::input!{
        mut s: Chars,
        a: Usize1, b: Usize1,
    }
    s.swap(a, b);
    println!("{}", s.into_iter().join(""))
}

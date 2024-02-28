use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }
    if s[s.len()-2..s.len()] == "er".chars().collect_vec() {
        println!("er");
    } else {
        println!("ist");
    }
}

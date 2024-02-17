use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        mut s: Chars,
    }
    let s_new = drop_match_paren(s);
    println!("{}", s_new.into_iter().join(""))
}

fn drop_match_paren(s: Vec<char>) -> Vec<char> {
    let n = s.len();
    let mut s_new: Vec<char> = Vec::with_capacity(n);
    let mut prev_open: Vec<usize> = vec![];
    for c in s {
        s_new.push(c);
        match c {
            '(' => {
                prev_open.push(s_new.len() - 1);
            }
            ')' => {
                if let Some(prev) = prev_open.pop() {
                    s_new.truncate(prev);
                }
            }
            _ => {}
        }
    }
    s_new
}

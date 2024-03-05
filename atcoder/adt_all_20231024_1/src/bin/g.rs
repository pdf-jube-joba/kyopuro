use std::collections::HashSet;

use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", if able(s) { "Yes" } else { "No" })
}

fn able(mut s: Vec<char>) -> bool {
    s.reverse();
    let mut stack = vec![];
    let mut hako: HashSet<char> = HashSet::new();
    while let Some(c) = s.pop() {
        match c {
            '(' => {
                stack.push(c);
            }
            ')' => {
                while let Some(c0) = stack.pop() {
                    if c0 == '(' {
                        break;
                    }
                    hako.remove(&c0);
                }
            }
            _ => {
                if hako.contains(&c) {
                    return false;
                }
                stack.push(c);
                hako.insert(c);
            }
        }
    }
    true
}

use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", if can_be_parindrome(s) { "Yes" } else { "No" })
}

fn can_be_parindrome(mut s: Vec<char>) -> bool {
    let i = s.iter().position(|c| *c != 'a');
    let j = s.iter().rposition(|c| *c != 'a');
    let (Some(i), Some(j)) = (i,j) else {
        debug_assert!(i.is_none() && j.is_none());
        return true;
    };

    if i > s.len() - 1 - j {
        return false;
    }

    s = s[i..=j].to_vec();

    (0..s.len()).all(|i| s[i] == s[s.len() - 1 - i])
}

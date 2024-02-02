use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let (i, j) = longest_palindrome(&s).unwrap();
    println!("{}", j - i)
}

fn longest_palindrome(s: &[char]) -> Option<(usize, usize)> {
    let n = s.len();
    (1..=n).rev().find_map(|diff| {
        (0..=n - diff).find_map(|i| {
            // 0 <= i <= i + diff <= n
            let s1 = s[i..i + diff].to_vec();
            let mut s2 = s1.clone();
            s2.reverse();
            if s1 == s2 {
                Some((i, i + diff))
            } else {
                None
            }
        })
    })
}

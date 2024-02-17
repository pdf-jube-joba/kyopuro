use proconio::marker::Chars;

fn main() {
    proconio::input! {
        _n: usize,
        s: Chars,
    }
    let n = s.len();
    let b = (0..n - 1).all(|i| s[i] != s[i + 1]);
    println!("{}", if b { "Yes" } else { "No" })
}

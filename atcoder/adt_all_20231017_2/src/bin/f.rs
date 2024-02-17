use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let pos = (0..n).find(|&i| s[i] != t[i]).unwrap_or(n) + 1;
    println!("{}", pos)
}

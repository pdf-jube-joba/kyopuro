use proconio::marker::Chars;

fn main() {
    proconio::input! {
        mut s: Chars,
    }
    s.sort();
    let b = s.iter().any(|c| c.is_ascii_lowercase())
        && s.iter().any(|c| c.is_ascii_uppercase())
        && s.windows(2).all(|v| v[0] != v[1]);
    println!("{}", if b { "Yes" } else { "No" });
}

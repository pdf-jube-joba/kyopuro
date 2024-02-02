use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars,
    }
    let is_prefix = s[0..n] == t[0..n];
    let is_suffix = s[0..n] == t[m-n..m];
    if is_prefix && is_suffix {
        println!("0")
    } else if is_prefix && !is_suffix {
        println!("1")
    } else if !is_prefix && is_suffix {
        println!("2")
    } else {
        println!("3")
    }
}

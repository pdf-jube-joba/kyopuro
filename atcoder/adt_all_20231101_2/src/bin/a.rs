use proconio::marker::Chars;

fn main() {
    proconio::input!{
        s: Chars,
    }
    let n = s.len();
    println!("{}", s[n/2])
}

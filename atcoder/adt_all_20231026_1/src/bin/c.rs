use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars, t: Chars,
    }
    let (ns, nt) = (s.len(), t.len());
    let b =  ns <= nt && (0..std::cmp::min(ns, nt)).all(|i| s[i] == t[i]);
    println!("{}", if b {"Yes"} else {"No"})
}

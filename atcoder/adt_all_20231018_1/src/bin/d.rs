use itertools::iproduct;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: [Chars; n],
    }
    let b = iproduct!(s.iter(), s.iter()).any(|(si, sj)|{
        if si == sj {
            return false;
        }
        let s: Vec<char> = si.iter().cloned().chain(sj.iter().cloned()).collect();
        let n = s.len();
        (0..n/2).all(|i| s[i] == s[n-i-1])
    });
    println!("{}", if b {"Yes"} else {"No"})
}

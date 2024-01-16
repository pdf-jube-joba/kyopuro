use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
    }
    let b = (0..n-1).any(|i|
        (s[i] == 'a' && s[i+1] == 'b') || (s[i] == 'b' && s[i+1] == 'a')
    );
    println!("{}", if b {"Yes"} else {"No"})
}

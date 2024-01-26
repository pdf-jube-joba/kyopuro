use proconio::marker::Chars;

fn main() {
    proconio::input!{
        s: Chars
    }
    let b = (0..8).all(|i| {
        // 1,3,...,15
        let c = s[2 * i + 1];
        eprintln!("{}", c);
        c == '0'});
    println!("{}", if b {"Yes"} else{"No"});
}

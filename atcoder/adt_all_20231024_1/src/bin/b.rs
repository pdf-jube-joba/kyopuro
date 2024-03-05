use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let s = s
        .into_iter()
        .filter(|c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .collect::<String>();
    println!("{}", s);
}

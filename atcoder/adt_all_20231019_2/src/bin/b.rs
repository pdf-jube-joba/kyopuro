use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let i = (0..s.len()).find(|i| s[*i].is_ascii_uppercase()).unwrap();
    println!("{}", i + 1)
}

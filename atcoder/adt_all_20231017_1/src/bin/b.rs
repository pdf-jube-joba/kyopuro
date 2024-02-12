use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let i = (0..=9).find(|i| s.iter().all(|c| c.to_string() != i.to_string())).unwrap();
    println!("{}", i)
}

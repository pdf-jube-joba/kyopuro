use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
    }
    let res = (0..=n - 3).find(|&i| s[i..i + 3] == vec!['A', 'B', 'C']);
    println!(
        "{}",
        if let Some(i) = res {
            (i+1).to_string()
        } else {
            "-1".to_string()
        }
    )
}

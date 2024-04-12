fn main() {
    proconio::input! {
        a: usize, b: usize, c: usize,
    }
    let n = (1..=1000).map(|i| c * i).find(|&i| a <= i && i <= b);
    println!(
        "{}",
        if let Some(n) = n {
            n.to_string()
        } else {
            "-1".to_string()
        }
    )
}

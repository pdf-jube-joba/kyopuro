fn main() {
    proconio::input! {
        n: usize,
    }
    let ne = if n % 5 <= 2 {
        5 * (n / 5)
    } else {
        5 * (n / 5) + 5
    };
    println!("{}", ne)
}

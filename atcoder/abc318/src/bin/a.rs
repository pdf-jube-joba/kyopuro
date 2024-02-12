fn main() {
    proconio::input! {
        n: usize, m: usize, p: usize,
    }
    let res = (1..=n).filter(|&d| d >= m && (d - m) % p == 0).count();
    println!("{res}")
}

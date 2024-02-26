fn main() {
    proconio::input! {
        x: usize, y: usize, n: usize,
    }
    let price: usize = if 3 * x <= y {
        n * x
    } else {
        (n / 3) * y + (n % 3) * x
    };
    println!("{}", price);
}

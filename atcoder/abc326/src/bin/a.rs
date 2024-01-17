fn main() {
    proconio::input! {
        x: usize, y: usize,
    }
    // x <= y <= x + 2 || x - 3 <= y <= x
    // <=> x - 3 <= y <= x + 2
    let b = x <= y + 3 && y <= x + 2;
    println!("{}", if b { "Yes" } else { "No" });
}

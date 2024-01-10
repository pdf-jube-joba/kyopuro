#[allow(non_snake_case)]
fn main() {
    proconio::input! {
        M: usize, D: usize,
        y: usize, m: usize, d: usize,
    }
    if d < D {
        println!("{} {} {}", y, m, d + 1);
    } else if m < M {
        println!("{} {} {}", y, m + 1, 1);
    } else {
        println!("{} {} {}", y + 1, 1, 1);
    }
}

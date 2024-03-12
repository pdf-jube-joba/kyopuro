fn main() {
    proconio::input! {
        n: usize, m: usize, x: usize, t: usize, d: usize,
    }
    let tall = if m >= x {
        t
    } else {
        t - (x - m) * d
    };
    println!("{}", tall);
}

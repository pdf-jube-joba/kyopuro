fn main() {
    proconio::input! {
        a: usize, b: usize,
    }
    let s: f64 = b as f64 / a as f64;
    let s = if (s * 10000_f64) as usize % 10 < 5 {
        (s * 1000_f64) as usize as f64 / 1000_f64
    } else {
        ((s * 1000_f64) as usize + 1) as f64 / 1000_f64
    };
    println!("{:.3}", s)
}

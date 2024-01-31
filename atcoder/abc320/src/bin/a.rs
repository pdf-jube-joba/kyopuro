fn main() {
    proconio::input! {
        a: usize, b: usize,
    }
    println!("{}", a.pow(b as u32) + b.pow(a as u32));
}

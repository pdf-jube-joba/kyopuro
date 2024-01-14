fn main() {
    proconio::input! {
        n: usize, x: usize,
        s: [usize; n],
    }
    println!("{}", s.into_iter().filter(|si| *si <= x).sum::<usize>())
}

fn main() {
    proconio::input! {
        n: usize, p: usize,
        a: [usize; n],
    }
    println!("{}", a.into_iter().filter(|ai| *ai < p).count())
}

fn main() {
    proconio::input! {
        n: usize, l: usize,
        a: [usize; n],
    }
    let num = a.into_iter().filter(|ai| l <= *ai).count();
    println!("{}", num);
}

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    let max = *a.iter().max().unwrap();
    let max2 = a.into_iter().filter(|ai| *ai != max).max().unwrap();
    println!("{}", max2);
}

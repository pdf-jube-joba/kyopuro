fn main() {
    proconio::input! {
        n: usize,
        mut x: [usize; 5 * n],
    }
    x.sort();
    let ave: f64 = x[n..4 * n].iter().sum::<usize>() as f64 / (3 * n) as f64;
    println!("{ave}");
}

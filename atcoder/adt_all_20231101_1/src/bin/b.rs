use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        h: [usize; n],
    }
    let m = h.into_iter().position_max().unwrap();
    println!("{}", m + 1)
}

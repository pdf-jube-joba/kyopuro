use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, p: Usize1, q: Usize1, r: Usize1, s: Usize1,
        mut a: [usize; n],
    }
    for i in p..=q {
        let j = r + i - p;
        a.swap(i, j)
    }
    println!("{}", a.into_iter().join(" "))
}

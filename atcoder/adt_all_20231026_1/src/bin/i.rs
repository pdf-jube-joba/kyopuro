use ac_library::{FenwickTree, ModInt998244353 as ModInt};
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, 
        a: [usize; n],
    }
}

// return for each k in 0..=n, (sum_{i in 0..k, j in 0..k} max(a[i], a[j])) / k^2
fn double_chance(a: Vec<usize>) -> Vec<ModInt> {
    let n = a.len();
    let mut asort = a.iter().cloned().enumerate().collect_vec();
    asort.sort_by_key(|(_, v)| *v);
    let mut ans = ModInt::new(0);
    // ft1[i] = if a[i] >= a[k] then 1 else 0
    let ft1: FenwickTree<usize> = FenwickTree::new(n + 1, 0);
    // ft2[i] = if a[i] >= a[k] then a[i] else 0
    let ft2: FenwickTree<usize> = FenwickTree::new(n + 1, 0);
    let presum = ModInt::new(0);
    for k in 0..n {
        let k = asort[k].0;

    }
    todo!()
}

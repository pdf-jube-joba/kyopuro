use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        a: [Usize1; 3 * n],
    }
    println!("{}", centers(a).into_iter().map(|i| i + 1).join(" "))
}

fn centers(a: Vec<usize>) -> Vec<usize> {
    let n = a.len() / 3;
    let mut nums = vec![vec![]; n];
    for (i, ai) in a.into_iter().enumerate() {
        nums[ai].push(i);
    }
    let mut v = (0..n).map(|i| (i, nums[i][1])).collect_vec();
    v.sort_by_key(|vi| vi.1);
    v.into_iter().map(|vi| vi.0).collect_vec()
}

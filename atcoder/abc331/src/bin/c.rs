use itertools::Itertools;
use itertools_num::ItertoolsNum;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", sum_of_lt(a).join(" "));
}

fn sum_of_lt(a: Vec<usize>) -> impl Iterator<Item = usize> {
    let n = a.len();
    let mut a_sorted = a.clone();
    a_sorted.sort();
    // sum_l[l in 0..=a.len()] = \sum_{i = 0}^{l - 1} a[i]
    let sum_l: Vec<usize> = std::iter::once(0).chain(a_sorted.iter().cumsum()).collect();
    let all_sum = sum_l[n];
    (0..n).map(move |i| {
        let ind = a_sorted.partition_point(|elm| *elm <= a[i]);
        all_sum - sum_l[ind]
    })
}

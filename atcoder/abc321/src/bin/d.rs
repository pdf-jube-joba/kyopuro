use itertools::Itertools;
use itertools_num::ItertoolsNum;

fn main() {
    proconio::input! {
        n: usize, m: usize, p: usize,
        a: [usize; n],
        b: [usize; m],
    }
    println!("{}", cost_sum(a, b, p));
}

fn cost_sum(mut a: Vec<usize>, mut b: Vec<usize>, p: usize) -> usize {
    let n = a.len();
    let m = b.len();
    b.sort();
    // sum_b[i] = sum_{j in 0..i} b[j]
    let sum_b: Vec<usize> = std::iter::once(0).chain(b.iter().cumsum()).collect_vec();
    a.into_iter()
        .map(|ai| {
            let j = b.partition_point(|bi| ai + *bi <= p);
            // k in 0..j => ai + b[k] <= p
            // k in j..m => ai + b[k] > p
            ai * j + sum_b[j] + (m - j) * p
        })
        .sum()
}

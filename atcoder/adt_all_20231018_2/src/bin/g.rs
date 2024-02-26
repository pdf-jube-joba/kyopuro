use itertools_num::ItertoolsNum;

fn main() {
    proconio::input! {
        n: usize, x: usize,
        ab: [(usize, usize); n],
    }
    println!("{}", min_time_trophy(ab, x));
}

fn min_time_trophy(ab: Vec<(usize, usize)>, x: usize) -> usize {
    let s: Vec<usize> = std::iter::once(0)
        .chain(ab.iter().map(|&(a, b)| a + b).cumsum())
        .collect();

    (0..std::cmp::min(ab.len(), x))
        .map(|i| s[i] + (ab[i].0 + ab[i].1) + ab[i].1 * (x - 1 - i))
        .min()
        .unwrap()
}

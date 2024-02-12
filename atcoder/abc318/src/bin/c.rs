use itertools::Itertools;
use itertools_num::ItertoolsNum;
use num_integer::Integer;

fn main() {
    proconio::input! {
        n: usize, d: usize, p: usize,
        f: [usize; n],
    }
    println!("{}", min_cost(f, d, p))
}

fn min_cost(mut f: Vec<usize>, d: usize, p: usize) -> usize {
    let n = f.len();
    f.sort();
    // sum[i] = sum_{j in 0..i} f[j]
    let sum: Vec<usize> = std::iter::once(0).chain(f.iter().cumsum()).collect_vec();

    let n_d_ceil = Integer::div_ceil(&n, &d);

    let min = (0..n_d_ceil)
        .map(|k| k * p + sum[n - k * d])
        .min();

    let min2 = n_d_ceil * p;

    if let Some(m) = min {
        std::cmp::min(m, min2)
    } else {
        min2
    }
}

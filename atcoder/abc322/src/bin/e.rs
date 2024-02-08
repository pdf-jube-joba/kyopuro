use std::ops::IndexMut;

use im_rc::HashMap;
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, k: usize, p: usize,
        c: [(usize, [usize; k]); n]
    }
    let res = min_cost(k, p, c);
    println!(
        "{}",
        if let Some(cost) = res {
            format!("{cost}")
        } else {
            "-1".to_string()
        }
    );
}

fn convert_to_base(p: usize, av: Vec<usize>) -> usize {
    av.into_iter()
        .enumerate()
        .map(|(i, pi)| pi * p.pow(i as u32))
        .sum()
}

fn convert_from_base(p: usize, mut a: usize, k: usize) -> Vec<usize> {
    let mut av = vec![];
    while a != 0 {
        av.push(a % p);
        a /= p;
    }
    av.extend(vec![0; k - av.len()]);
    av
}

fn min_cost(k: usize, p: usize, c: Vec<(usize, Vec<usize>)>) -> Option<usize> {
    let n = c.len();
    let truncate = |i| -> usize { std::cmp::min(p, i) };
    let to_b = |av: Vec<usize>| -> usize { convert_to_base(p + 1, av) };
    let fr_b = |a: usize| -> Vec<usize> { convert_from_base(p + 1, a, k) };
    // c[i] = cost[i], a^i[0..k]
    // dp[i in 0..n][(a_0,...,a_{k-1}) in (0..=p)^k ] = min { sum_{y_i = true}  cost[i]
    //      | y_{i in 0..k}: bool, a_j = min(p, sum_{y_i = true} a^i[j] ) }
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; to_b(vec![p; k]) + 1]];
    // base case: i = 0
    dp[0][0] = Some(0);
    if to_b(c[0].1.to_owned()) != 0 {
        dp[0][to_b(c[0].1.to_owned())] = Some(c[0].0);
    }

    for i in 1..n {
        // not choose c[i] case: dp[i][a] <- dp[i-1][a] for all a
        dp.push(dp[i - 1].to_owned());
        // choose c[i] case
        for a in 0..to_b(vec![p; k]) + 1 {
            let Some(cost_pre) = dp[i - 1][a] else {
                continue;
            };
            let av_next = fr_b(a)
                .into_iter()
                .enumerate()
                .map(|(j, aj)| truncate(aj + c[i].1[j]))
                .collect_vec();
            let cost = cost_pre + c[i].0;
            let a_next = to_b(av_next);
            dp[i][a_next] = if let Some(c) = dp[i][a_next] {
                Some(std::cmp::min(c, cost))
            } else {
                Some(cost)
            };
        }
    }

    dp[n - 1][to_b(vec![p; k])]
}

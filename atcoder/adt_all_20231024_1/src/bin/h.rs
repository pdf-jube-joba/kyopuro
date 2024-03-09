use itertools::Itertools;
use num_integer::lcm;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        mut s: [Chars; n],
    }
    let ans = lcps(s);
    for ans in ans {
        println!("{}", ans);
    }
}

// max of i s.t. s0[0..i] == s1[0..i]
fn lcp(s0: &Vec<char>, s1: &Vec<char>) -> usize {
    let n = std::cmp::min(s0.len(), s1.len());
    (0..n).find(|&i| s0[i] != s1[i]).unwrap_or(n)
}

fn lcps(s: Vec<Vec<char>>) -> Vec<usize> {
    let n = s.len();
    let mut s = s.into_iter().enumerate().collect_vec();
    s.sort_by_key(|(i, si)| si.clone());
    let mut collected: Vec<(usize, usize)> = (0..n)
        .map(|i| {
            let prev_lcp = if i == 0 {
                None
            } else {
                Some(lcp(&s[i].1, &s[i - 1].1))
            };
            let next_lcp = if i == n - 1 {
                None
            } else {
                Some(lcp(&s[i].1, &s[i + 1].1))
            };
            match (prev_lcp, next_lcp) {
                (Some(p), Some(n)) => (s[i].0, std::cmp::max(p, n)),
                (Some(v), None) | (None, Some(v)) => (s[i].0, v),
                (None, None) => unreachable!(),
            }
        })
        .collect();

    collected.sort_by_key(|(i, lcpi)| *i);
    collected.into_iter().map(|(i, lcpi)| lcpi).collect()
}

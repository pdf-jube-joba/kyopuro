use std::cmp::Reverse;

use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [usize; m],
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|char| match char {
                    'o' => true,
                    'x' => false,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let count = min_solve_num(a, s);

    for i in count {
        println!("{}", i);
    }
}

fn min_solve_num(a: Vec<usize>, s: Vec<Vec<bool>>) -> Vec<usize> {
    let n = s.len();
    let m = a.len();

    // now_point[i in 0..n] = \sum_{j in 0..m | player i solved problem j} (point of problem j)
    let now_point: Vec<usize> = (0..n)
        .map(|i| {
            (0..m)
                .filter_map(|j| if s[i][j] { Some(a[j]) } else { None })
                .sum::<usize>()
            + (i + 1) // bonus point
        })
        .collect_vec();
    let max_point = *now_point.iter().max().unwrap();

    // under assumptions in contest, now_point has no duplicate element

    (0..n)
        .map(|i| {
            // for each player i
            let mut not_solved = (0..m).filter(|j| !s[i][*j]).collect_vec();
            not_solved.sort_by_key(|j| Reverse(a[*j]));
            // not_solved = { j in 0..m | player i not solved problem j } ordered from highest to lowest point

            let mut point_i = now_point[i];
            let mut ans = 0;
            while point_i < max_point {
                point_i += a[not_solved[ans]];
                ans += 1;
            }
            // if it holds that point_i == max_point when break the loop
            // <=> ans == 0 and player i achieve higherst score
            // else i.e. point_i > max_point
            ans
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_test() {
        let res = min_solve_num(vec![100], vec![vec![false], vec![false]]);
        assert_eq!(res, vec![1, 0])
    }
}

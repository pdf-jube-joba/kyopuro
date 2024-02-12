fn main() {
    proconio::input! {
        n: usize,
    }
    let mut d: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        proconio::input! {
            di: [usize; n-i-1],
        }
        d.push(di);
    }
    println!("{}", max_cost(d))
}

fn max_cost(d: Vec<Vec<usize>>) -> usize {
    let n = d[0].len() + 1;
    let mut dp: Vec<usize> = vec![0; 1 << n];
    for i in 1..(1 << n) {
        let first_index = (0..n).rev().find(|idx| ((1 << idx) & i) != 0).unwrap();
        let r = (0..first_index)
            .rev()
            .filter_map(|idx| {
                let j = if (1 << idx) & i != 0 {
                    i - (1 << first_index) - (1 << idx)
                } else {
                    return None;
                };
                Some(dp[j] + d[idx][first_index - idx - 1])
            })
            .max();
        let non_choose_cost = dp[i - (1 << first_index)];
        if let Some(r) = r {
            dp[i] = std::cmp::max(r, non_choose_cost);
        } else {
            dp[i] = non_choose_cost;
        }
    }

    dp[(1 << n) - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        // let d = vec![vec![3, 3, 3, 3], vec![3, 3, 3], vec![3, 3], vec![3]];
        // let r = max_cost(d);
        // assert_eq!(r, 6);

        let d = vec![vec![5, 3, 3, 3], vec![3, 3, 3], vec![3, 3], vec![3]];
        let r = max_cost(d);
        assert_eq!(r, 8);

        // let d = vec![vec![5, 5, 3, 3], vec![3, 3, 3], vec![3, 3], vec![3]];
        // let r = max_cost(d);
        // assert_eq!(r, 8);
    }
}

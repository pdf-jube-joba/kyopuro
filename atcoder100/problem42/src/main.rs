fn main() {
    proconio::input! {
        n: usize, m: usize,
        d: [usize; n],
        c: [usize; m],
    }

    println!("{}", minimize(&d, &c));

}

fn minimize(d: &[usize], c: &[usize]) -> usize {
    let n = d.len();
    let m = c.len();
    // dp[i][j] で i 日目に j にいるときの疲労度の最小
    let mut dp: Vec<Vec<Option<usize>>> = Vec::with_capacity(m);
    for i in 0..m {
        dp.push({
            (0..=n)
            .map(|t|{
                if i > 0 {
                    let h1 = dp[i-1][t];
                    let h2 = if t > 0 {
                        dp[i-1][t-1].map(|l| l + c[i] * d[t-1])
                    } else {None};
                    match (h1, h2) {
                        (Some(l1), Some(l2)) => Some(std::cmp::min(l1, l2)),
                        (Some(l1), _) => Some(l1),
                        (_, Some(l2)) => Some(l2),
                        _ => None,
                    }
                } else {
                    if t == 0 {Some(0)} else {None}
                }
            })
            .collect()
        });
    }
    (*dp.last().unwrap().last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test(){
        let d = vec![10, 25, 15];
        let c = vec![50, 30, 15, 40, 30];
        assert_eq!(minimize(&d, &c), 1125);
    }
}
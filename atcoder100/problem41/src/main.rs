fn main() {
    proconio::input! {
        d: usize, n: usize,
        t: [usize; d],
        abc: [(usize, usize, usize); n],
    }

    println!("{}", maximize(&t, &abc));

}

fn maximize(t: &[usize], abc: &[(usize, usize, usize)]) -> usize {
    let d = t.len();
    let n = abc.len();

    // dp[i][j] で i 日目に tj を着るときの派手さの max
    let mut dp: Vec<Vec<Option<usize>>> = Vec::with_capacity(d);
    for i in 0..d {
        dp.push(abc.iter().enumerate().map(|(j, &(aj, bj, cj))|{
            if aj <= t[i] && t[i] <= bj { Some(
                if i > 0 {
                    abc.iter().enumerate()
                    .filter_map(|(j2, &(_, _, cj2))| {
                        dp[i-1][j2].map(|l|l + cj2.abs_diff(cj))
                    })
                    .max().unwrap()
                } else { 0 }
            )} else {None}
        }).collect());
    }
    *dp.last().unwrap().iter().flatten().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_test(){
        let t = vec![31, 27, 35];
        let abc = vec![
            (20, 25, 30),
            (23, 29, 90),
            (21, 35, 60),
            (28, 33, 40),
        ];
        assert_eq!(maximize(&t, &abc), 80);
    }
}
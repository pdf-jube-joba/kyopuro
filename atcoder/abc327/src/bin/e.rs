use superslice::Ext;

fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n],
    }
    println!("{}", max_of_sum_k(p));
}

fn max_of_sum_k(p: Vec<usize>) -> f64 {
    let n = p.len();
    // $dp[i in 0.. n][j in 0..=i] = max {sum_{k in 0..=j} (0.9)^(j-k) p[l_k] | 0 <= l_0 < ... < l_j <= i }$
    let mut dp: Vec<Vec<f64>> = vec![vec![]; n];
    dp[0].push(p[0] as f64);
    for i in 1..n {
        // dp[i-1][j in 0..=i] is valid
        // j = 0 case
        for j in 0..=i {
            let new_dp_i_j = {
                if j == 0 {
                    dp[i - 1][0].max(p[i] as f64)
                } else if j <= i - 1 {
                    dp[i - 1][j].max(0.9 * dp[i - 1][j - 1] + p[i] as f64)
                } else {
                    0.9 * dp[i - 1][i - 1] + p[i] as f64
                }
            };
            dp[i].push(new_dp_i_j);
        }
    }

    let mut s = vec![];
    let mut c = 1_f64;
    for j in 0..n {
        s.push(c);
        c = 0.9 * c + 1_f64;
    }

    dp.pop().unwrap().into_iter().enumerate().map(|(j, mj)|{
        mj / s[j] - (1200_f64 / ((j + 1) as f64).sqrt())
    }).reduce(|a, b| a.max(b)).unwrap()

}

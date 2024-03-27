fn main() {
    proconio::input! {
        n: usize,
        xy: (usize, usize),
        ab: [(usize, usize); n],
    }
    let ans = strange_lunchbox(xy, ab);
    println!(
        "{}",
        if let Some(p) = ans {
            p.to_string()
        } else {
            "-1".to_string()
        }
    )
}

fn strange_lunchbox((x, y): (usize, usize), ab: Vec<(usize, usize)>) -> Option<usize> {
    let n = ab.len();
    // dp[i in 0..=n][j in 0..=x][k in 0..=y] = ab[0..i] でちょうど (j, k) 個 を買うのに必要な最小値、ただし j=x,k=y のときは x以上とy以上とする。
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![std::usize::MAX; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for j in 0..=x {
            for k in 0..=y {
                // dp[i][j][k] -> dp[i+1][j+a[i]][k+b[i]]
                let j_ = std::cmp::min(j + ab[i].0, x);
                let k_ = std::cmp::min(k + ab[i].1, y);
                dp[i + 1][j_][k_] = std::cmp::min(dp[i + 1][j_][k_], dp[i][j][k].saturating_add(1));
                dp[i + 1][j][k] = std::cmp::min(dp[i + 1][j][k], dp[i][j][k])
            }
        }
    }

    if dp[n][x][y] == std::usize::MAX {
        None
    } else {
        Some(dp[n][x][y])
    }
}

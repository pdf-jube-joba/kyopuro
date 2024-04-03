use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [usize; n],
        c: [usize; n],
        x: [usize; m],
    }
    println!("{}", wish_list(a, c, x));
}

fn wish_list(a: Vec<usize>, c: Vec<usize>, x: Vec<usize>) -> usize {
    let n = a.len();
    // let m = x.len();
    let x: HashSet<usize> = x.into_iter().collect();
    
    // cost[i in 0..n][j in 0..=i] = min { C[i-j], ..., C[i] }
    let mut cost: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        cost[i].push(c[i]);
    }
    for i in 0..n {
        for j in 0..i {
            let cn = std::cmp::min(cost[i][j], c[i - (j + 1)]);
            cost[i].push(cn);
        }
    }
    
    eprintln!("{cost:?}");    

    let mut dp: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n + 1]; n];
    dp[0][0] = 0;
    for i in 0..n - 1 {
        for j in 0..=i {
            if !x.contains(&i) {
                dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j]);
            }
            dp[i + 1][j + 1] = std::cmp::min(
                dp[i + 1][j + 1],
                dp[i][j].saturating_add(a[i + 1] + cost[i + 1][j]),
            );
        }
    }
    eprintln!("{dp:?}");
    (0..=n).map(|i| dp[n - 1][i]).min().unwrap()
}

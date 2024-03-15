fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    println!("{}", make_bipartite(a, b));
}

// vertex = (0..=n)
// edge = { (0, i): weight a[i-1] } for i in 1..=n
//      or {(i, i + 1 mod (n+1)): weight b[i-1]}) for i in 1..=n
// return min cost of sum of weight of deleted edges to make bipartite gra@h
fn make_bipartite(a: Vec<usize>, b: Vec<usize>) -> usize {
    let n = a.len();
    // dp[i in 1..=n][j = 0 | 1][k = 0 | 1] = min ..of.. which is restricted to
    // - vertex is (0..=i)
    // - vertex 0 is colored 0, vertex 1 is colored j, vertex i is colored k
    // dp[i + 1][j][k] = min (dp[i][j][k] + b[i-1], dp[i][j][1-k]) + if k = 0 {a[i]} else {0}
    let mut dp: Vec<((usize, usize), (usize, usize))> = vec![((0, 0), (0, 0)); n + 1];
    // dp[0], dp[1][0][1] and dp[1][1][0] is nonsense
    dp[1] = ((a[0], std::usize::MAX), (std::usize::MAX, 0)); // dp[1][0][0] = a[0], dp[1][1][1] = 0
    for i in 2..=n {
        let bi = b[i - 2];
        let ai = a[i - 1];
        // eprint!("{i} {:?} {} {}", dp[i - 1], bi, ai);
        dp[i].0 .0 = std::cmp::min(dp[i - 1].0 .0.saturating_add(bi), dp[i - 1].0 .1) + ai;
        dp[i].0 .1 = std::cmp::min(dp[i - 1].0 .0, dp[i - 1].0 .1.saturating_add(bi));
        dp[i].1 .0 = std::cmp::min(dp[i - 1].1 .0.saturating_add(bi), dp[i - 1].1 .1) + ai;
        dp[i].1 .1 = std::cmp::min(dp[i - 1].1 .0, dp[i - 1].1 .1.saturating_add(bi));
        // eprintln!(" {:?}", dp[i])
    }
    let v = vec![
        dp[n].0 .0 + b[n - 1],
        dp[n].0 .1,
        dp[n].1 .0,
        dp[n].1 .1 + b[n - 1],
    ];
    v.into_iter().min().unwrap()
}

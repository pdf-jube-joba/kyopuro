fn main() {
    proconio::input! {
        n: usize, d: usize,
        w: [usize; n],
    }
    let sum: usize = w.iter().sum();
    let min_square = min_square(d, w);
    let v = (d * min_square - sum.pow(2)) as f64 / d.pow(2) as f64;
    println!("{}", v);
}

// return min {d の袋に {w[i] | i in 0..n } を分けたときの、それぞれの袋の重さの2乗の和}
fn min_square(d: usize, w: Vec<usize>) -> usize {
    let n = w.len();
    // dp[i \in 0..d][S \subset 0..n] = min { i"+1" の袋に {w[i] | i in S} を分けたときの..}
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 1 << n]; d];
    // base case dp[S][0] = (\sum_{s \in S} w[s])^2
    for s in 0..1 << n {
        let sum: usize = (0..n)
            .map(|i| if s & (1 << i) == 0 { 0 } else { w[i] })
            .sum();
        dp[0][s] = sum.pow(2);
    }
    // recurstion case dp[i+1][S] = min {dp[i][S - T] + dp[0][T] | T: subset of S}
    for i in 0..d-1 {
        for s in 0..1 << n {
            let t_enum = EnumSubOfSub::new(s);
            let min = t_enum.map(|t| dp[i][t ^ s] + dp[0][t]).min().unwrap();
            dp[i + 1][s] = min;
        }
    }
    dp[d - 1][(1 << n) - 1]
}

#[derive(Debug, Clone, PartialEq)]
struct EnumSubOfSub {
    s: usize,
    t: Option<usize>,
}

impl EnumSubOfSub {
    fn new(s: usize) -> Self {
        Self { s, t: Some(s) }
    }
}

impl Iterator for EnumSubOfSub {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.t.take()?;
        if t > 0 {
            self.t = Some((t - 1) & self.s);
        }
        Some(t)
    }
}

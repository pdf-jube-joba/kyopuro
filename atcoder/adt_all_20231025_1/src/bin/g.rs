use ac_library::ModInt998244353 as ModInt;
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    println!("{}", between_two_arrays(a, b))
}

const M: usize = 3000;

fn between_two_arrays(a: Vec<usize>, b: Vec<usize>) -> ModInt {
    let n = a.len();
    let mut dp: Vec<Vec<ModInt>> = vec![vec![ModInt::new(0); M + 1]; n + 1];
    dp[0] = vec![ModInt::new(1); M + 1];
    for i in 1..=n {
        for j in 1..=M {
            dp[i][j] = {
                if a[i - 1] <= j && j <= b[i - 1] {
                    dp[i][j - 1] + dp[i - 1][j]
                } else {
                    dp[i][j - 1]
                }
            };
        }
    }
    eprintln!("{:?}", dp[1].iter().filter(|dpij| **dpij != ModInt::new(0)).collect_vec());
    // eprintln!("{}", dp[0][0]);
    // eprintln!("{}", dp[0][1]);
    // eprintln!("{}", dp[1][0]);
    // eprintln!("{}", dp[1][1]);
    dp[n][M]
}

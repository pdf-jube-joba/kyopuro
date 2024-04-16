use ac_library::ModInt998244353 as ModInt;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", make_10_again(a))
}

fn make_10_again(a: Vec<usize>) -> ModInt {
    let n = a.len();
    let mut dp: Vec<Vec<ModInt>> = vec![vec![ModInt::new(0); 11]; n + 1];
    dp[0][0] = ModInt::new(1);
    for i in 0..n {
        for t in 0..=10 {
            dp[i + 1][t] = (dp[i][t] * ModInt::new(a[i]))
                + (1..=a[i])
                    .take_while(|&dt| t >= dt)
                    .map(|dt| dp[i][t - dt])
                    .sum::<ModInt>();
        }
    }
    eprintln!("{dp:?}");
    let all = a
        .iter()
        .fold(ModInt::new(1), |acc, ai| acc * ModInt::new(*ai));
    dp[n][10] / all
}

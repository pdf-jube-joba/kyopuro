use ac_library::ModInt998244353 as ModInt;
use itertools::{iproduct, Itertools};
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    println!("{}", yet_another_grid_task(s))
}

fn yet_another_grid_task(s: Vec<Vec<bool>>) -> ModInt {
    let (n, m) = (s.len(), s[0].len());
    let t = (0..m)
        .map(|j| (0..n).find(|&i| s[i][j]).unwrap_or(n))
        .collect_vec();

    // let mut num: Vec<Vec<ModInt>> = vec![vec![ModInt::new(0); n + 1]; m + 1];
    // num[0][n] = ModInt::new(1);

    // for j in 0..m {
    //     for i in 0..=t[j] {
    //         num[j + 1][i] = (i.saturating_sub(1)..=n).map(|i_| num[j][i_]).sum();
    //     }
    // }

    let mut dp: Vec<Vec<ModInt>> = vec![vec![ModInt::new(0); n + 1]; m + 1];
    dp[0] = vec![ModInt::new(1); n + 1];
    for j in 0..m {
        let mut pre = ModInt::new(0);
        for i in (0..=t[j]).rev() {
            pre += dp[j][i.saturating_sub(1)];
            dp[j + 1][i] = pre;
        }
    }

    dp[m][0]

}

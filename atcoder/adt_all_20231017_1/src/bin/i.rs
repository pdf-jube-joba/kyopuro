use ac_library::ModInt998244353 as ModInt;
use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        x: Chars,
    }
    let x = x
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();
    println!("{}", sum_of_d(x));
}

fn sum_of_d(s: Vec<usize>) -> ModInt {
    let n = s.len();
    let mut dp_f: Vec<ModInt> = vec![ModInt::new(1), ModInt::new(s[0])];
    let mut dp_g: Vec<ModInt> = vec![ModInt::new(1), ModInt::new(1) + dp_f[1]];
    for i in 2..=n {
        dp_f.push(ModInt::new(s[i - 1]) * dp_g[i - 1] + ModInt::new(10) * dp_f[i - 1]);
        dp_g.push(dp_g[i - 1] + dp_f[i]);
    }
    dp_f[n]
}

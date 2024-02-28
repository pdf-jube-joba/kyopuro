use ac_library::ModInt1000000007 as ModInt;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", num_subst_1skip(s));
}

fn num_subst_1skip(s: Vec<char>) -> ModInt {
    let n = s.len();
    // dp[i in 0..=n] = num of 1 skip substr s.t. uses s[i-1]
    let mut dp: Vec<ModInt> = vec![ModInt::new(0)];
    dp[0] = ModInt::new(1);
    for i in 1..n {
        let k = (0..i).rev().find(|&j| s[i] == s[j]).unwrap_or(0);
        dp.push(dp[k..i - 1].iter().sum());
    }
    // (0..=n).map(|i| dp[i]).sum()
    todo!("わからなくなった。")
}

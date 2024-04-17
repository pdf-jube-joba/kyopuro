use ac_library::ModInt998244353 as ModInt;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", make_10_again(a))
}

const M: usize = 10;

fn make_10_again(a: Vec<usize>) -> ModInt {
    let n = a.len();

    let all = a
        .iter()
        .fold(ModInt::new(1), |acc, ai| acc * ModInt::new(*ai));

    let mut dp: Vec<Vec<ModInt>> = vec![vec![ModInt::new(0); 1 << (M + 1)]; n + 1];
    dp[0][1] = ModInt::new(1);
    let mask: usize = (1 << (M + 1)) - 1;
    for i in 0..n {
        for s in 0..(1 << (M + 1)) {
            let tmp = dp[i][s];
            for t in 1..=M.min(a[i]) {
                let s2 = (s | (s << t)) & mask;
                dp[i + 1][s2] += tmp;
            }
            dp[i + 1][s] += tmp * a[i].saturating_sub(M);
        }
    }
    let num: ModInt = (0..(1 << M)).map(|s| dp[n][s | (1 << M)]).sum();

    num / all
}

fn naive(a: Vec<usize>) -> ModInt {
    let n = a.len();
    let mask: usize = (1 << (M + 1)) - 1;
    let all = a
        .iter()
        .fold(ModInt::new(1), |acc, ai| acc * ModInt::new(*ai));
    let num: ModInt = std::iter::successors(Some(vec![1; n]), |s| {
        let i = (0..n).find(|&i| s[i] < a[i])?;
        let mut s = s.clone();
        s[i] += 1;
        for k in 0..i {
            s[k] = 1;
        }
        Some(s)
    })
    .filter(|v| {
        let mut s = 1;
        for i in 0..n {
            if v[i] <= M {
                s = (s | (s << v[i])) & mask;
            }
        }
        s & (1 << 10) != 0
    })
    .inspect(|v| eprintln!("{v:?}"))
    .count()
    .into();
    num / all
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        let a = vec![11];
        assert_eq!(make_10_again(a.clone()), naive(a));
        let a = vec![11, 11];
        assert_eq!(make_10_again(a.clone()), naive(a));
        let a = vec![1, 10, 100];
        assert_eq!(make_10_again(a.clone()), naive(a));
    }
}

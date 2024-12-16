use ac_library::ModInt998244353 as ModInt;
use num_integer::div_ceil;

fn main() {
    proconio::input! {
        n: usize, p: usize,
    }
    println!("{}", critical_hit(n, p))
}

fn critical_hit(n: usize, p: usize) -> ModInt {
    let p = ModInt::new(p) / ModInt::new(100);
    let nonp = ModInt::new(1) - p;
    let mut p_pow = vec![ModInt::new(1); n + 1];
    let mut nonp_pow = vec![ModInt::new(1); n + 1];
    for i in 0..n {
        p_pow[i + 1] = p_pow[i] * p;
        nonp_pow[i + 1] = nonp_pow[i] * nonp;
    }

    let mut fact = vec![ModInt::new(1); n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
    }

    let k1 = (1..=n)
        .skip_while(|&k| k < n - k)
        .map(|k| {
            (fact[k] / (fact[n - k] * fact[2 * k - n]) * p_pow[n - k] * nonp_pow[2 * k - n]) * k
        })
        .sum::<ModInt>();
    let k2 = (1..=n)
        .skip_while(|&k| (k - 1) < n - k)
        .map(|k| {
            (fact[k - 1] / (fact[n - k] * fact[2 * k - n - 1])
                * p_pow[n - k + 1]
                * nonp_pow[2 * k - n - 1])
                * k
        })
        .sum::<ModInt>();
    k1 + k2
}

use ac_library::ModInt998244353 as ModInt;
use num_integer::div_ceil;

fn main() {
    proconio::input! {
        n: usize, p: usize,
    }
    println!("{}", critical_hit(n, p))
}

fn critical_hit(n: usize, p: usize) -> ModInt {
    let p100 = ModInt::new(p) / ModInt::new(100);
    let mut p_pow = vec![ModInt::new(1)];
    let mut nonp_pow = vec![ModInt::new(1)];
    for _ in 0..n {
        let p = *p_pow.last().unwrap();
        p_pow.push(p * p100);
        let p = *nonp_pow.last().unwrap();
        nonp_pow.push(p * (ModInt::new(1) - p100));
    }

    let mut fact = vec![ModInt::new(1)];
    for i in 1..=n {
        let fact_prev = *fact.last().unwrap();
        fact.push(fact_prev * i);
    }
    let comb = |n: usize, k: usize| -> ModInt { fact[n] * fact[k] / fact[n + k] };

    let p1 = |k: usize| -> ModInt { comb(k, n - k) * p_pow[n - k] * nonp_pow[2 * k - n] };

    let p2 =
        |k: usize| -> ModInt { comb(k - 1, n - k) * p_pow[n - k + 1] * nonp_pow[2 * k - n - 1] };

    (div_ceil(n, 2)..=n).map(|k| p1(k) * k).sum::<ModInt>()
        + (div_ceil(n + 1, 2)..=n).map(|k| p2(k) * k).sum::<ModInt>()
}

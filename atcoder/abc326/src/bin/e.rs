use std::{
    iter::Sum,
    ops::{Add, Mul},
};

const MOD: usize = 998_244_353;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ModInt(usize);

impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % MOD)
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % MOD)
    }
}

impl Sum for ModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = ModInt(0);
        for item in iter {
            sum = sum + item;
        }
        sum
    }
}

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", prob(a).0);
}

// pow
fn pow(mut a: ModInt, mut m: usize) -> ModInt {
    let mut res: ModInt = ModInt(1);
    while m > 0 {
        if m % 2 == 1 {
            res = res * a;
        }
        a = a * a;
        m >>= 1;
    }
    res
}

// inv
fn inv(a: ModInt) -> ModInt {
    pow(a, MOD - 2)
}

fn prob(a: Vec<usize>) -> ModInt {
    let n = a.len();
    let mut p = vec![ModInt(1)];
    let mut sum: ModInt = ModInt(1);

    for i in 0..n {
        // sum = \sum_{j = 0}^{i} p[j]
        // p[i + 1] = sum * (1/ n)
        let pj = inv(ModInt(n)) * sum;
        sum = sum + pj;
        // set p[i + 1]
        p.push(pj);
    }

    (0..n).map(|i| (ModInt(a[i]) * p[i + 1])).sum()
}

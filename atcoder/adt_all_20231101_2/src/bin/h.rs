use ac_library::ModInt998244353 as ModInt;
use num_traits::Pow;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        k: usize,
    }
    println!("{}", sugoroku((n, m, k)));
}

fn pow(m: ModInt, k: usize) -> ModInt {
    if k == 0 {
        ModInt::new(1)
    } else if k % 2 == 0 {
        let m = pow(m, k / 2);
        m * m
    } else {
        pow(m, k - 1) * m
    }
}

fn sugoroku((n, m, k): (usize, usize, usize)) -> ModInt {
    let mut now: Vec<ModInt> = vec![ModInt::new(0); n + 1];
    now[0] = ModInt::new(1);
    for _ in 0..k {
        let mut next: Vec<ModInt> = vec![ModInt::new(0); n + 1];
        for no in 0..n {
            for mo in 1..=m {
                let to = if no + mo <= n {
                    no + mo
                } else {
                    n - (no + mo - n)
                };
                next[to] += now[no];
            }
        }
        next[n] += now[n];
        now = next;
    }
    now[n] / pow(ModInt::new(m), k)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn powt() {
        for i in 0..20 {
            assert_eq!(pow(ModInt::new(2), i), ModInt::new(2.pow(i)));
        }
    }
}

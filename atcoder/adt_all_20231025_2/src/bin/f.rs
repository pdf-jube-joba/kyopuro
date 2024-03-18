use ac_library::ModInt998244353 as ModInt;

fn main() {
    proconio::input! {
        n: usize,
    }
    println!("{}", digitnum(n));
}

// f(1) + ... + f(n)
// = f(1) + .. + f()
fn digitnum(n: usize) -> ModInt {
    let mut ans = ModInt::new(0);
    let mut i = 10;
    while n >= i {
        ans += f_sumdigit(i - 1);
        i *= 10;
    }
    ans += f_sumdigit(n);
    ans
}

// f(10^i) + f(10^i + 1) ... + f(b) where 10^i <= b < 10^(i+1)
// = 1 + 2 + ... + f(b) = f(b) * (f(b) + 1) / 2
fn f_sumdigit(b: usize) -> ModInt {
    let fb = f(b);
    fb * (fb + 1) / 2
}

fn f(n: usize) -> ModInt {
    let mut i = 1;
    while n >= i {
        i *= 10;
    }
    i /= 10;
    ModInt::new(n - i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        assert_eq!(f(1), ModInt::new(1));
        assert_eq!(f(9), ModInt::new(9));
        assert_eq!(f(10), ModInt::new(1));
        assert_eq!(f(23), ModInt::new(14));

        assert_eq!(f_sumdigit(1), ModInt::new(1));
        assert_eq!(f_sumdigit(3), ModInt::new(6));
        assert_eq!(f_sumdigit(2), ModInt::new(3));
        assert_eq!(f_sumdigit(9), ModInt::new(9 * (9 + 1) / 2));
        assert_eq!(f_sumdigit(10), ModInt::new(1));
        assert_eq!(f_sumdigit(23), ModInt::new(14 * (14 + 1) / 2));

        for i in 1..30 {
            assert_eq!(digitnum(i), (1..=i).map(|j| f(j)).sum::<ModInt>());
        }
    }
}

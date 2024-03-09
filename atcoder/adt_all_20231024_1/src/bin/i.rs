use ac_library::ModInt998244353 as ModInt;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); q],
    }
    // B[i][j] = if (i + j): even then m * i + j + 1 else 0
    for (a, b, c, d) in abcd {
        println!("{}", sq1((n, m), (a, b + 1, c, d + 1)));
    }
}

// sum_{i in 0..n} a + di
fn sum_arith_seq(a: ModInt, d: ModInt, n: usize) -> ModInt {
    a * n + d * n * (n.saturating_sub(1)) / ModInt::new(2)
}

// sum_{i in 0..a, j in 0..b} B[i][j]
fn sq0((n, m): (usize, usize), (a, b): (usize, usize)) -> ModInt {
    let (a0, b0) = (a / 2, b / 2);
    let (mm, nn) = (ModInt::new(m), ModInt::new(n));
    match ((a % 2 == 0), (b % 2 == 0)) {
        (true, true) => {
            let s0 = sum_arith_seq(ModInt::new(3), ModInt::new(4), b0);
            sum_arith_seq(mm * b0 + s0, mm * 4 * b0, a0)
        }
        (false, true) => {
            sq0((n, m), (2 * a0, 2 * b0)) + sum_arith_seq(mm * a0 * 2 + 1, ModInt::new(2), b0)
        }
        (true, false) => {
            sq0((n, m), (2 * a0, 2 * b0)) + sum_arith_seq(ModInt::new(b0 * 2 + 1), mm * 2, a0)
        }
        (false, false) => {
            sq0((n, m), (2 * a0, 2 * b0))
                + sum_arith_seq(mm * a0 * 2 + 1, ModInt::new(2), b0)
                + sum_arith_seq(ModInt::new(b0 * 2 + 1), mm * 2, a0)
                + 2 * a0 * m
                + 2 * b0
                + 1
        }
    }
}

// sum_{i in a..b, j in c..d} B[i][j]
fn sq1((n, m): (usize, usize), (a, b, c, d): (usize, usize, usize, usize)) -> ModInt {
    sq0((n, m), (b, d)) + sq0((n, m), (a, c)) - sq0((n, m), (a, d)) - sq0((n, m), (b, c))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sm() {
        assert_eq!(
            sum_arith_seq(ModInt::new(3), ModInt::new(4), 1),
            ModInt::new(3)
        );
        assert_eq!(
            sum_arith_seq(ModInt::new(3), ModInt::new(4), 2),
            ModInt::new(3 + 3 + 4)
        );
    }
    #[test]
    fn sq() {
        assert_eq!(sq0((2, 2), (0, 0)), ModInt::new(0));
        assert_eq!(sq0((2, 2), (0, 2)), ModInt::new(0));
        assert_eq!(sq0((2, 2), (2, 0)), ModInt::new(0));
        assert_eq!(sq0((2, 2), (2, 2)), ModInt::new(5));
        assert_eq!(sq0((3, 3), (1, 1)), ModInt::new(1));
        assert_eq!(sq0((3, 3), (2, 2)), ModInt::new(6));
        assert_eq!(sq0((3, 3), (3, 3)), ModInt::new(25));
        assert_eq!(sq0((3, 3), (3, 2)), ModInt::new(13));
        assert_eq!(sq0((3, 3), (2, 1)), ModInt::new(1));

        assert_eq!(sq1((3, 3), (0, 0, 0, 0)), ModInt::new(0));
        assert_eq!(sq1((3, 3), (0, 1, 0, 1)), ModInt::new(1));
        assert_eq!(sq1((3, 3), (0, 2, 0, 1)), ModInt::new(1));
        assert_eq!(sq1((3, 3), (0, 3, 0, 1)), ModInt::new(8));
    }
}

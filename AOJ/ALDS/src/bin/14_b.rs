use std::ops::{Add, AddAssign, Mul, Sub};

// mod_int should < 2^64 so that mul is well-def
#[derive(Debug, Clone, Copy, PartialEq)]
struct ModInt {
    mod_int: u128,
    a: u128,
}

const MOD_BASES: [ModInt; 2] = [
    ModInt {
        mod_int: (1 << 61) - 1,
        a: 1_000_000_007,
    },
    ModInt {
        mod_int: 1_000_000_007,
        a: 29,
    },
];

impl ModInt {
    fn new(mod_int: u128, a: u128) -> Self {
        Self {
            mod_int,
            a: (a % mod_int),
        }
    }
    fn modulo(&self) -> u128 {
        self.mod_int
    }
}

impl Add for ModInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        debug_assert!(self.mod_int == other.mod_int);
        let mod_int = self.mod_int;
        ModInt {
            mod_int,
            a: (self.a + other.a) % mod_int,
        }
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(self.mod_int == rhs.mod_int);
        self.a += rhs.a;
        self.a %= self.mod_int;
    }
}

impl Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        debug_assert!(self.mod_int == other.mod_int);
        let mod_int = self.mod_int;
        Self {
            mod_int,
            a: (self.a + mod_int - other.a) % mod_int,
        }
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        debug_assert!(self.mod_int == other.mod_int);
        let mod_int = self.mod_int;
        Self {
            mod_int,
            a: (self.a * other.a) % mod_int,
        }
    }
}

// a^(2^b)
fn pow_2(mut a: ModInt, b: usize) -> ModInt {
    for i in 0..b {
        a = a * a;
    }
    a
}

// a^b
fn pow(a: ModInt, b: usize) -> ModInt {
    let mut pow: ModInt = ModInt {
        mod_int: a.mod_int,
        a: 1,
    };
    for i in 0..64 {
        if b & (1 << i) != 0 {
            pow = pow * pow_2(a, i);
        }
    }
    pow
}

// base.mod_int == hash.mod_ind
#[derive(Debug, Clone, PartialEq)]
struct RollingHash {
    base: ModInt,
    hash: ModInt,
}

impl RollingHash {
    fn new(base: ModInt) -> Self {
        let hash = ModInt {
            mod_int: base.mod_int,
            a: 0,
        };
        RollingHash { base, hash }
    }
    fn hash(&self) -> ModInt {
        self.hash
    }
    fn hashing(&mut self, h: &[u8]) {
        let n = h.len();
        let mod_int = self.base.modulo();
        let mut sum: ModInt = ModInt::new(mod_int, 0);
        for i in 0..n {
            sum += ModInt::new(mod_int, h[i] as u128) * pow(self.base, n - 1 - i);
        }
        self.hash = sum;
    }
    fn update(&mut self, next: ModInt) {
        self.hash = next;
    }
}

fn find_substr(t: &[u8], p: &[u8], bases: &[ModInt]) -> Vec<usize> {
    // debug_assert!({
    //     println!("{} {}", t.len(), p.len());
    //     true
    // });
    let t_len = t.len();
    let p_len = p.len();

    if t_len < p_len {
        return vec![];
    }

    let mut hashes: Vec<RollingHash> = vec![];
    let mut target_hashes: Vec<RollingHash> = vec![];

    let mut v = vec![];

    for &base in bases {
        let mut t_start_hash = RollingHash::new(base);
        t_start_hash.hashing(&t[0..p_len]);
        hashes.push(t_start_hash);

        let mut p_hash = RollingHash::new(base);
        p_hash.hashing(&p[..]);
        target_hashes.push(p_hash);
    }

    // eprintln!("t{:x}", target_hashes[0].hash.a);

    for i in 0..=t_len - p_len {
        // eprintln!("[{}..{}] {:x}", i, i + p_len, hashes[0].hash.a);
        // hash is hash of hash[i..i+p_Len]
        if hashes == target_hashes {
            v.push(i);
        }
        if i != t_len - p_len {
            for (j, &base) in bases.iter().enumerate() {
                let mod_int = base.modulo();
                let next_hash = base * hashes[j].hash()
                    - pow(base, p_len) * ModInt::new(mod_int, t[i] as u128)
                    + ModInt::new(mod_int, t[i + p_len] as u128);
                hashes[j].update(next_hash);
            }
        }
    }
    v
}

fn main() {
    let (t, p) = input();
    let v = find_substr(&t, &p, &MOD_BASES);
    for i in v {
        println!("{}", i);
    }
}

fn input() -> (Vec<u8>, Vec<u8>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim().as_bytes().to_vec();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let p = buf.trim().as_bytes().to_vec();

    (t, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn nice_base() -> Vec<ModInt> {
        vec![ModInt {
            mod_int: 1 << 16,
            a: 1 << 8,
        }]
    }
    #[test]
    fn modint_test() {
        let tests = vec![
            (0, 0, 0),
            (1, 1, 2),
            (5, 4, 9),
            (5, 5, 0),
            (5, 6, 1),
            (20, 11, 1),
            ((1 << 61) - 1, (1 << 61) - 1, (2 * (1 << 61)) - 2),
        ];
        for (a, b, c) in tests {
            let a = ModInt::new(10, a);
            let b = ModInt::new(10, b);
            let c = ModInt::new(10, c);
            assert_eq!(a + b, c);
        }
    }
    #[test]
    fn unoverflowing() {
        let a: ModInt = ModInt::new((1 << 61) - 1, (1 << 61) - 2);
        let b: ModInt = ModInt::new((1 << 61) - 1, (1 << 61) - 3);
        let _ = a + b;
        let _ = a - b;
        let _ = b - a;
        let _ = a * b;

        let a: ModInt = ModInt::new((1 << 61) - 1, (1 << 61) - 2);
        for b in 0..=100_usize {
            let c = pow(a, b);
            assert_eq!(
                {
                    let mut pow: ModInt = ModInt::new((1 << 61) - 1, 1);
                    for i in 0..b {
                        pow = pow * a;
                    }
                    pow
                },
                c
            )
        }
    }
    #[test]
    fn str_find_test() {
        let t = b"a";
        let p = b"a";
        let v = find_substr(t, p, &nice_base());
        assert_eq!(v, vec![0]);

        let t = b"aaa";
        let p = b"a";
        let v = find_substr(t, p, &nice_base());
        assert_eq!(v, vec![0, 1, 2]);

        let t = b"abab";
        let p = b"c";
        let v = find_substr(t, p, &nice_base());
        assert_eq!(v, vec![]);

        let t = b"ababab";
        let p = b"ab";
        let v = find_substr(t, p, &nice_base());
        assert_eq!(v, vec![0, 2, 4]);
    }
    #[test]
    fn long_test1() {
        let t = b"001100010001".repeat(1_000_000);
        let p = b"0011";
        let v = find_substr(&t, p, &nice_base());
        for (i, vi) in v.into_iter().enumerate() {
            assert_eq!(12 * i, vi)
        }
    }
}

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

const MODINT: u128 = (1 << 61) - 1;
const MOD_BASES: [u128; 2] = [100_000_007, 100_000_009];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModInt261(u128);

impl From<u128> for ModInt261 {
    fn from(u: u128) -> ModInt261 {
        ModInt261(u)
    }
}

impl From<u8> for ModInt261 {
    fn from(u: u8) -> ModInt261 {
        ModInt261(u as u128)
    }
}

impl From<usize> for ModInt261 {
    fn from(u: usize) -> ModInt261 {
        ModInt261(u as u128)
    }
}

impl Add for ModInt261 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ModInt261((self.0 + other.0) % MODINT)
    }
}

impl AddAssign for ModInt261 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.0 %= MODINT;
    }
}

impl Sub for ModInt261 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ModInt261((self.0 + MODINT - other.0) % MODINT)
    }
}

impl SubAssign for ModInt261 {
    fn sub_assign(&mut self, other: Self) {
        self.0 += MODINT;
        self.0 -= other.0;
        self.0 %= MODINT;
    }
}

impl Mul for ModInt261 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModInt261((self.0 * other.0) % MODINT)
    }
}

impl MulAssign for ModInt261 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.0 %= MODINT;
    }
}

// a^(2^b)
fn pow_2_mod(mut a: ModInt261, b: usize) -> ModInt261 {
    for i in 0..b {
        a = a * a;
    }
    a
}

// a^b
fn pow_mod(a: ModInt261, b: usize) -> ModInt261 {
    let mut pow = 1_usize.into();
    for i in 0..64 {
        if b & (1 << i) != 0 {
            pow *= pow_2_mod(a, i);
        }
    }
    pow
}

fn find_substr_2_61_1(hw: Vec<Vec<u8>>, rc: Vec<Vec<u8>>, bases: &[u128]) -> Vec<(usize, usize)> {
    let (h, w, r, c) = (hw.len(), hw[0].len(), rc.len(), rc[0].len());

    if h < r || w < c {
        return vec![];
    }

    let b = bases.len();
    let bases: Vec<ModInt261> = bases.iter().map(|base| (*base).into()).collect();
    let bases_pow_c: Vec<ModInt261> = (0..b).map(|b| pow_mod(bases[b], c)).collect();
    let bases_pow_rc: Vec<ModInt261> = (0..b).map(|b| pow_mod(bases_pow_c[b], r)).collect();

    // hash of rc[0..r][0..c]
    // O(rc)
    let target_hashes: Vec<ModInt261> = bases
        .iter()
        .map(|base| {
            let mut sum: ModInt261 = 0_usize.into();
            for i in 0..r {
                for j in 0..c {
                    sum *= *base;
                    sum += rc[i][j].into();
                }
            }
            sum
        })
        .collect::<Vec<_>>();

    // dbg!(&target_hashes);

    // bases b での (hw_{l,k})_{0 \leq l < r, k = j} の部分の hash （を適当に次数ずらした） \sum_{0 \leq l < r} hw_{l,j} * b^{c * (r-1 -l)} を事前に計算する
    // vertical_slice_hashes[b][j] = above hash of j-th vertical slice over base base[b]
    // O(rw)
    let mut vertical_slice_hashes: Vec<Vec<ModInt261>> = (0..b)
        .map(|b| {
            (0..w)
                .map(|j| {
                    let mut sum = 0_usize.into();
                    for i in 0..r {
                        sum *= bases_pow_c[b];
                        sum += hw[i][j].into();
                    }
                    sum
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&vertical_slice_hashes);

    let mut v = vec![];

    // dbg!("start");

    for i in 0..=h - r {
        // hash of hw[i..i+r][0..0+c]
        // O(c)
        let mut hashes: Vec<ModInt261> = (0..b)
            .map(|b| {
                let mut sum: ModInt261 = 0_usize.into();
                for k in 0..c {
                    sum *= bases[b];
                    sum += vertical_slice_hashes[b][k];
                }
                sum
            })
            .collect();

        // dbg!("i", i, &hashes);

        // hash of hw[i..i+r][j..j+c]
        // O(w-c)
        for j in 0..=w - c {
            if hashes == target_hashes {
                v.push((i, j));
            }

            // dbg!("j", j, &hashes);
            if j != w - c {
                // compute hash of hw[i..i+r][j+1..j+c+1] from hw[i..i+r][j..j+c]
                for b in 0..b {
                    hashes[b] *= bases[b];
                    hashes[b] -= bases_pow_c[b] * vertical_slice_hashes[b][j];
                    hashes[b] += vertical_slice_hashes[b][j + c];
                }
            }
        }

        if i != h - r {
            for j in 0..w {
                for b in 0..b {
                    vertical_slice_hashes[b][j] *= bases_pow_c[b];
                    vertical_slice_hashes[b][j] -= bases_pow_rc[b] * hw[i][j].into();
                    vertical_slice_hashes[b][j] += hw[i + r][j].into();
                }
            }
        }
    }

    v
}

fn main() {
    let (hw, rc) = input();
    let ans = find_substr_2_61_1(hw, rc, &MOD_BASES);
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}

fn input() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let (h, _w) = {
        let v = buf
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let hw = (0..h)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            buf.trim().as_bytes().to_vec()
        })
        .collect::<Vec<_>>();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let (r, _c) = {
        let v = buf
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let rc = (0..r)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            buf.trim().as_bytes().to_vec()
        })
        .collect::<Vec<_>>();

    (hw, rc)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn nice_base() -> Vec<u128> {
        vec![1 << 8]
    }
    fn col<'a>(v: Vec<impl IntoIterator<Item = &'a u8>>) -> Vec<Vec<u8>> {
        v.into_iter()
            .map(|v| v.into_iter().cloned().collect())
            .collect()
    }
    fn assert_contain(v: Vec<(usize, usize)>, a: Vec<(usize, usize)>) {
        for a in a {
            assert!(v.contains(&a));
        }
    }
    #[test]
    fn find_subpattern_test() {
        let hw = vec![b"1".to_vec()];
        let rc = vec![b"1".to_vec()];
        let v = find_substr_2_61_1(hw, rc, &MOD_BASES);
        assert_contain(v, vec![(0, 0)]);

        let hw = vec![b"2".to_vec()];
        let rc = vec![b"1".to_vec()];
        let v = find_substr_2_61_1(hw, rc, &MOD_BASES);
        assert_contain(v, vec![]);

        let hw = vec![b"".to_vec()];
        let rc = vec![b"1".to_vec()];
        let v = find_substr_2_61_1(hw, rc, &MOD_BASES);
        assert_contain(v, vec![]);
    }
    #[test]
    fn find_subpattern_nontrivial_test() {
        let tests = vec![
            ((vec![b"00", b"00"], vec![b"1"]), vec![]),
            ((vec![b"10", b"00"], vec![b"1"]), vec![(0, 0)]),
            ((vec![b"01", b"00"], vec![b"1"]), vec![(0, 1)]),
            ((vec![b"00", b"10"], vec![b"1"]), vec![(1, 0)]),
            ((vec![b"00", b"01"], vec![b"1"]), vec![(1, 1)]),
        ];
        for ((hw, rc), expect) in tests {
            let v = find_substr_2_61_1(col(hw), col(rc), &MOD_BASES);
            assert_contain(v, expect);
        }
    }
    #[test]
    fn large_test() {
        let max = 1_000;
        let rc_max = 5;
        let hw = vec![
            b"01110".repeat(max / 5).to_vec(),
            b"11011".repeat(max / 5).to_vec(),
            b"11011".repeat(max / 5).to_vec(),
            b"11011".repeat(max / 5).to_vec(),
            b"11011".repeat(max / 5).to_vec(),
        ]
        .into_iter()
        .cycle()
        .take(max)
        .collect();
        let rc = vec![b"01110", b"11011", b"11011", b"11011", b"11011"];
        let v = find_substr_2_61_1(hw, col(rc), &MOD_BASES);

        use std::collections::HashSet;
        use std::iter::FromIterator;

        let set: HashSet<(usize, usize)> = std::collections::HashSet::from_iter(v);

        println!("start");

        for i in 0..max/5 {
            for j in 0..max/5 {
                assert!(set.contains(&(5 * i, 5 * j)));
            }
        }
    }
}

use std::ops::{Add, AddAssign, Mul, Sub};

const MODINT: u128 = (1 << 32);

// a < 2^61 - 1
#[derive(Debug, Clone, Copy, PartialEq)]
struct Mod2pow61m1 {
    a: u128,
}

impl Add for Mod2pow61m1 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Mod2pow61m1 {
            a: (self.a + other.a) % MODINT,
        }
    }
}

impl AddAssign for Mod2pow61m1 {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.a %= MODINT;
    }
}

impl Sub for Mod2pow61m1 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            a: (self.a + MODINT - other.a) % MODINT,
        }
    }
}

impl Mul for Mod2pow61m1 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            a: (self.a * other.a) % MODINT,
        }
    }
}

impl From<u8> for Mod2pow61m1 {
    fn from(u: u8) -> Self {
        Self { a: u.into() }
    }
}

impl From<usize> for Mod2pow61m1 {
    fn from(u: usize) -> Self {
        Self { a: u as u128 }
    }
}

fn pow(a: Mod2pow61m1, b: Mod2pow61m1) -> Mod2pow61m1 {
    Mod2pow61m1 {
        a: a.a.saturating_pow(b.a as u32) % MODINT,
    }
}

fn find_substr(t: &[u8], p: &[u8], base: usize) -> Vec<usize> {
    // debug_assert!({
    //     println!("{} {}", t.len(), p.len());
    //     true
    // });
    let t_len = t.len();
    let p_len = p.len();

    let base: Mod2pow61m1 = base.into();

    if t_len < p_len {
        return vec![];
    }

    let mut v = vec![];

    let hasing = |c: &[u8]| {
        debug_assert!({ c.len() == p_len });
        let mut sum: Mod2pow61m1 = 0_usize.into();
        for i in 0..p_len {
            sum += Mod2pow61m1::from(c[i]) * pow(base, (p_len - 1 - i).into());
        }
        sum
    };

    let mut hash = hasing(&t[0..p_len]);
    let target_hash = hasing(&p[..]);
    eprintln!("--{:x}", target_hash.a);

    for i in 0..=t_len - p_len {
        eprintln!("[{}..{}] {:x}", i, i+p_len, hash.a);
        // hash is hash of hash[i..i+p_Len]
        if hash == target_hash {
            v.push(i);
        }
        if i != t_len - p_len {
            hash = hash * base - pow(base, p_len.into()) * t[i].into() + t[i + p_len].into();
        }
    }
    v
}

fn main() {
    let (t, p) = input();
    let base = 1_000_000_007;
    let v = find_substr(&t, &p, base);
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
    #[test]
    fn unoverflowing() {
        let a: Mod2pow61m1 = ((1 << 61) - 2_usize).into();
        let b: Mod2pow61m1 = ((1 << 61) - 3_usize).into();
        let _ = a + b;
        let _ = a - b;
        let _ = b - a;
        let _ = a * b;
        let c = pow(a, b);
        assert_eq!(
            Mod2pow61m1 {
                a: (std::u128::MAX % MODINT)
            },
            c
        )
    }
    #[test]
    fn str_find_test() {
        let t = b"a";
        let p = b"a";
        let v = find_substr(t, p, 1 << 8);
        assert_eq!(v, vec![0]);

        let t = b"aaa";
        let p = b"a";
        let v = find_substr(t, p, 1 << 8);
        assert_eq!(v, vec![0, 1, 2]);

        let t = b"abab";
        let p = b"c";
        let v = find_substr(t, p, 1 << 8);
        assert_eq!(v, vec![]);

        let t = b"ababab";
        let p = b"ab";
        let v = find_substr(t, p, 1 << 8);
        assert_eq!(v, vec![0, 2, 4]);
    }
    #[test]
    fn long_test1() {
        let t = b"001100010001".repeat(1_000_000);
        let p = b"0011";
        let v = find_substr(&t, p, 1 << 8);
        for (i, vi) in v.into_iter().enumerate() {
            assert_eq!(12 * i, vi)
        }
    }
    #[test]
    fn long_test2() {
        let t =
        b"bcacbabbbbbabacbccaacbcacbcbcbbaabcaabbbababccaccbcaacacbacbbaccaaaabcccbcbaccacaaabbbbcbbaacabcbcaccccacbbbbcbccbbaacbcbbbbcabacbbcaaaacbabbbbcbcaccbcbaaaabbccabccccccccbacccacaccbbbcabbcbabbcabbabbabcaacabcbaccaaabcbaacaabbbcbabcabaabbcacaacbaccbbbbabccccccabbaacbcabbcbbcbacabbcabbabbabbaaacaabbaccbbaaaacbbabcaaaccbaacbbbcbcaaccbcabcbbbaaaaabbcacaabcbbbbbcbcbcccbcbcbcaccabaababcbabaccccbbaaaaccccbbababcbcabcaaccbcbbbacababbcabcbbbcabaaccaacccbabccbccbcacaabacabbbcbcabcaccaaabbabbacbbcccacaabbcbbbccbcbccacaccbaabbcabcbbcaacccacbabacaaaabcaacbccaaabbaabaaccacababbbbcccbcabbaacbcacaccaccabbaacccaabcacacbccabaaaaacaacaabbbbbaacbcaccbbbabbbcbbabbbcccaabbaacbaabbcbccccabbaabbccabccbbcacccaaacbabbabaaaabbbabbbabbacaabbbcbbcbcbacacbaccaaabbcccacccbbacbcbbbbacbabccaccaabccacaacccbacaaabcccbbcaabaabbbcbbabcbbbbccaaabccbbbaccbaaaacbcbaaacccbbcbbabbacccbababcaccbbaacbbccbaaaacbbbcaacbbacaaaabcbbabacacababbabbbbccacbbbbccbaaacabacccabababcbabbabbbaaaacbbacbabbabbbaaaabaacccabacaacbabcbaacacbbbcbbacacaaccbbcabaaabbaacbccbbbababcacbbaaaccbcbcccaaababcacacaccbcbcacaccccbcabbcccabbcaccaaaaacabbaababbccaaacabcbcbbcbaababcbbaaabcaaaaccbbbccbbbccbbbcacbcacbbacbbccccbbaababaccaabbbbbcbbcaaaabcabccabbacbacccaaacbbaaaaaaabaabacabcccabbacacabaccbababcabaccbabcbbbacaaacacccaacbaacccbbaabcacccbbaabaabcabbabaacbcbbbbcbccabbabcabcccbbccabccbcabcaaacccacbbacccbbbbcabcaaccccbbbbcbbcbcacbaabccabaccccaacacbbbaabbabccbcbbaccccbacbbcbbcbbbbabcbccccabcbbacaacbcccacbbcbcaaabccaacaabacabbacbcbbabacacaccbbbbabcaccccccbbabababbbcabccbbacccabbcbbcbbcaaababbccabbcabccccccbbbaabcbaccbabbbbbcbcacabbabacacbacbcaaaacbaccaaaccaabbcabacbcbccbbccbcccabbabbbaacaaacabccacbacabbcabccbabcbcacbcabaaccaccabaabbbbaaaaccccababaabbbcbbabbaccabbacbaacacbcbaacbbaacacbabacaccbacbcaccbbbbcbbcaabbbbbcaaccbbbbaccaabcbbabbcababcccbbcbbaabcaccabcabcaccbbabcbacbaccaacacaaacabacccacbabaababbbcaaaabcbbabcccaaabaabcaaacccaaababcbabaacaabaccaaabbaccbbbacabbbcabbcbcbccabbabbaaabaaaccacccbaccbbcbabbacccacbbacaacbacbcbbbccbbaaccccabbcbabaaabbbcbabccaacabbbccacbababbbcccbaccaacbaccbccacbbbcaabcacacbcacaccaaaabcccaaaccbbccbacbcaccaccbbaabaacaacbbaaacbbcabbccbbccacbccbaccabcaccabaaabababbaaacbbbaabcabbbcbccaaacbaabcacaabaaabcabbcacbbaabcccababbcbacbbbacbbcbbabbcaaccccbbaacccbbccacaabcabbcbbabcbcbabacabaabbbbbcacbcccabbcccbaaacccacaccaacccaaaabacabbbbbaabbabaacbbbaccaacbaccbacccaabbbbacccbabcccacccabbacabccbcabacacaabababbcbacbccabacbcabacbcbbcacaaccbbcaaabcabcbcacbabcabbbacaacbacccbcccbbcabaacbbacbbbaacabaacacbcccbbaabbabbccabaccbaabacbaccabbaabbbccaacbaccbccbcccaaccaccbbabbcacaaaaacbaccabbaccccbbabbaccbaccbabbabbccbcbacccbbcaabcbbbbcabacbbbbaabbcbccbbacbcabacabcbcccccbbacbbccabcbccbcabcccccbcbbcbbbccbacaccbaacccabaabccbababbcacbaabaaccaaaabcacbacbbccbbaacabcaccbaaabbbbbccacbbbccbcbaacabaabcbaabbccaacaccacbcbbbbacbcabaacabbbacaaaccbccbbcbabacbccbcacbaacbcababbcbacccabbbcccabbccbbabaabcbacccaaaccaabbaabccbbbacbbbbbabcabcacaababaaabacbaacabcbcaabbcbcbacbbaaabaaacbcaccacacbaacbccabaccbbacbaacaaccbbccbacaacacccacccccbcbbccaaacacbccacbaacaacabcabbcccbacacabcabcabcaabccccaabcbcbbcacacaaaccbbccacabbcbabbacbccaacbaabcacbabbaabaaabbcbcbaaacaaaccaabcbbccababaabbbabaacabaacbaabbaaccccccbcabccababbbbcabaccbcbcbbcabccaacacbacabbbcaaccabcccbcaacaccacccbcbccbabaccccbabaaaccaacbabccbcbbaaacacaabcccabbabaaaaacabcbabccabacbbaacacabbbbbbcbacabbaaacbaccacbbbbcbccaabcbacaaccaacabccbcbccabbcacbababbccabacaaaacaabbcaacbaaaabaccaaaccbaabbbccacaabaacbccbabaaacababcbcacbbcbabcaccbaaacabbaccbabbcaacaabcacaccacbccaaaaccbbcccbccbabcaacbcabcbbabbbcccbbcabbacbabbbbabbcbcaabbbaabaaaccaaacbbbbcaabbcaababcbabcbcccbcbbcabbbaabbacaaaabacaccccbbcbabaccbbacbccaccababcbaabbaaacabacaabbbccbbbccbaabbcaaacbbacbaccaccbbbcbaaaaccbaaaababbcacccabbbbbacaabbbbcbaaccbabacacacccbbacabababacacbcaccaaacaaabacbabbbbbbcabbaababbbbbbcbaabacabbbaccccacccaaccbbaccbaabbbaaaaccbcccccabcabacbbcaacbbcababbbbbbbbbabaaccacbbacaaaaabaacacbababaccababaaaabccbaacabaacbcbacabbabbbcbbabbcbababbcbaaaacacaabcbcbcbbbccabbbccbacbcaccbbcbcacbbccbbbcbbbcccbaccaabaaccccabaabcabccacabaaabcaabababccabccaacbcaabcbbacbacacbaacabccbabaccacbaccccabcaaaabcbcacccbbbcabcacbcaccababcaaabcaabbabbcacbcbaabccccccbccbaccbbacababacacbaccaacccbccaccbabacaacaacababbcbabbbbbaaaabacaabbcbacabcbcacacbbaaccbaacbccacacccbaccbcabcabcbacbaaabbacaaabaabbabacaabbcbabacbcccbaabacccaacbbaacabcbacabbaaabaccbbbcabbbababcababbccbcacbbaacbaacbcabbbbbbabcbbbbaacaabbcacbbccabcaaccbaabbababcbccbcbbababcaaacbbcaababacbccacacbbabaacbbacaacbcbbcaccabaaaccabcccaacccccbcbbaaacaabcbcbbaacaccaccabcaaacababcaacaccacbccbbababaacaabbaaccbbbccabcababacaabcbaacaacbcacccbbaabacbcabacbcccbaabcbaccaccacabccabbbaaabcbbccccccaacababaaacbbcbcbaabacbabacbbcabbabcacaabcacbbbbaccaccbacaaaabcbcbbabbccbcbabacbacabbbcbccaabccbaabacaabcaabbccababacaacaacbcccbabccccbccbacabcababbababccaccabccacbaabbbcacbccaaccbacabaaacbccbabaacbbbbcacbbbcaabbabcacabbcbccacbcbcbcabacabcacaacccbbacbaaabbbcaaccacaacccbabcacbaaababbbaacbcabccbcaabccabccbbbcccbabbbbbbabbcbbbabcccccacacbbcabbaacccbaaaccbabbcbbcbccbcbaabbcaccaccbaacaba";
        let p = b"bcabccab";
        let v = find_substr(t, p, 1 << 16);
        assert_eq!(v.contain(vec![1191, 3160]))
    }
}

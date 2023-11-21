const MODINT: u128 = (1 << 61) - 1;
const MOD_BASES: [u128; 2] = [100_000_007, 100_000_009] ;

fn find_substr_2_61_1(t: &[u8], p: &[u8], bases: &[u128]) -> Vec<usize> {
    let t_len = t.len();
    let p_len = p.len();

    if t_len < p_len {
        return vec![];
    }

    let mut hashes = bases.iter().map(|base| {
        let mut sum = 0;
        for i in 0..p_len {
            sum *= base;
            sum += t[i] as u128;
            sum %= MODINT;
        }
        sum
    }).collect::<Vec<_>>();

    let target_hashes = bases.iter().map(|base| {
        let mut sum = 0;
        for i in 0..p_len {
            sum *= base;
            sum += p[i] as u128;
            sum %= MODINT;
        }
        sum
    }).collect::<Vec<_>>();

    let base_pows = bases.iter().map(|base| {
        // a^(2^b)
        fn pow_2_mod(mut a: u128, b: usize) -> u128 {
            for i in 0..b {
                a = (a * a) % MODINT;
            }
            a
        }

        // a^b
        fn pow_mod(a: u128, b: usize) -> u128 {
            let mut pow = 1;
            for i in 0..64 {
                if b & (1 << i) != 0 {
                    pow *= pow_2_mod(a, i);
                    pow %= MODINT;
                }
            }
            pow
        }
        
        pow_mod(*base, p_len)
    }).collect::<Vec<_>>();

    let mut v = vec![];

    for i in 0..=t_len-p_len {
        if hashes == target_hashes {
            v.push(i);
        }
        if i != t_len-p_len {
            for (j, base) in bases.iter().enumerate() {
                hashes[j] *= base;
                hashes[j] += MODINT;
                hashes[j] -= (base_pows[j] * (t[i] as u128)) % MODINT;
                hashes[j] += t[i+p_len] as u128;
                hashes[j] %= MODINT;
            }
        }
    }

    v
}

fn main() {
    let (t, p) = input();
    let v = find_substr_2_61_1(&t, &p, &MOD_BASES);
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
    fn str_find_test() {
        let t = b"a";
        let p = b"a";
        let v = find_substr_2_61_1(t, p, &MOD_BASES);
        assert_eq!(v, vec![0]);

        let t = b"aaa";
        let p = b"a";
        let v = find_substr_2_61_1(t, p, &MOD_BASES);
        assert_eq!(v, vec![0, 1, 2]);

        let t = b"abab";
        let p = b"c";
        let v = find_substr_2_61_1(t, p, &MOD_BASES);
        assert_eq!(v, vec![]);

        let t = b"ababab";
        let p = b"ab";
        let v = find_substr_2_61_1(t, p, &MOD_BASES);
        assert_eq!(v, vec![0, 2, 4]);
    }
    #[test]
    fn long_test1() {
        let t = b"001100010001".repeat(100_000);
        let p = b"0011";
        let v = find_substr_2_61_1(&t, p, &MOD_BASES);
        println!("end");
        for (i, vi) in v.into_iter().enumerate() {
            assert_eq!(12 * i, vi)
        }
    }
}

const MODINT: u64 = (1 << 61) - 1;
const BASE: u64 = (1 << 8);

fn find_sub(t: &[u8], p: &[u8]) -> Vec<usize> {
    let t_len = t.len();
    let p_len = p.len();
    let mut v = vec![];

    if t_len < p_len {
        return v;
    }

    let hashing = |c: &[u8]| {
        assert!(c.len() == p_len);
        (0..p_len).map(|i| mod_prod(c[i] as u64, BASE.pow((p_len - 1 - i) as u32) as u64)).sum()
    };

    // start
    let mut hash: u64 = hashing(&t[0..p_len]);
    let tgt_hash: u64 = hashing(&p[..]);
    // eprintln!("t: {:?} {:x}", &p[..], tgt_hash);

    for i in p_len..=t_len {
        // eprintln!("  [{}..{}], {:?} {:x}", i - p_len, i, &t[i-p_len..i], hash);
        // hash is hash of t[i-p_len..i]
        if hash == tgt_hash {
            v.push(i - p_len);
        }
        if i != t_len {
            // eprintln!("{:x} {:x} {:x}", hash * BASE, t[i-p_len] as u64 * BASE.pow(p_len as u32), t[i] as u64);
            hash = mod_plus(mod_minus(hash * BASE, t[i-p_len] as u64 * BASE.pow(p_len as u32)), t[i] as u64);
            // now hash is hash of t[i-p_len+1..i+1]
        }
    }
    v
}

fn mod_minus(mut i: u64, mut j: u64) -> u64 {
    j %= MODINT;
    i += MODINT;
    // i > j
    (i - j) % MODINT
}

fn mod_plus(i: u64, j: u64) -> u64 {
    (i + j) % MODINT
}

fn mod_prod(i: u64, j: u64) -> u64 {
    (i * j) % MODINT
}

fn main() {
    let (t, p) = input();
    for i in find_sub(&t, &p) {
        println!("{}", i);
    }
}

fn input() -> (Vec<u8>, Vec<u8>) {
    use std::convert::TryFrom;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim().to_string().into_bytes();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let p = buf.trim().to_string().into_bytes();

    (t, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let t = b"ababab";
        let p = b"ab";
        let v = find_sub(t, p);
        assert_eq!(v, vec![0, 2, 4]);

        let t = b"aabaaa";
        let p = b"aa";
        let v = find_sub(t, p);
        assert_eq!(v, vec![0, 3, 4]);

        let t = b"xyzz";
        let p = b"yz";
        let v = find_sub(t, p);
        assert_eq!(v, vec![1]);

        let t = b"iooooioooi";
        let p = b"ioooi";
        let v = find_sub(t, p);
        assert_eq!(v, vec![5]);
    }
    #[test]
    fn long_str() {
        let t = b"ab".repeat(1_000_000);
        let p = "ab";
        let v = find_sub(&t, p.as_bytes());

        for (i, vi) in v.into_iter().enumerate() {
            assert_eq!(2 * i, vi);
        }

    }
}

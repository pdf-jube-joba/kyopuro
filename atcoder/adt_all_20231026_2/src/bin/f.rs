use std::cmp::Reverse;

fn main() {
    proconio::input! {
        n: usize,
    }
    println!("{}", select_mul(n));
}

fn n2v(mut n: usize) -> Vec<usize> {
    let mut v = vec![];
    while n != 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

fn v2n(v: Vec<usize>) -> usize {
    v.into_iter()
        .enumerate()
        .map(|(i, vi)| 10_usize.pow(i as u32) * vi)
        .sum()
}

fn select_mul(n: usize) -> usize {
    let digits = n2v(n);
    (0..(1 << digits.len()) - 1)
        .filter_map(|subset| {
            let mut a = vec![];
            let mut b = vec![];
            for i in 0..digits.len() {
                if 1 << i & subset != 0 {
                    a.push(digits[i]);
                } else {
                    b.push(digits[i]);
                }
            }
            a.sort();
            b.sort();
            if a.iter().all(|i| *i == 0) || b.iter().all(|i| *i == 0) {
                return None;
            }
            Some(v2n(a) * v2n(b))
        })
        .max()
        .unwrap()
}

use std::vec;

fn main() {
    proconio::input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    println!("{}", minimize(&a, k));

}

fn minimize(a: &[usize], k: usize) -> usize {
    fn convert(n: usize, a: usize) -> Vec<bool> {
        (0..n).map(|i|{
            a & (1 << i) != 0
        }).collect()
    }
    let n = a.len();
    (0..(1 << n)).map(|i|{convert(n, i)}).filter_map(|bits|{
        if !(bits.iter().filter(|b|**b).count() < k) {
            let mut b: Vec<usize> = vec![0; n];
            let mut now = 0;
            (0..n).for_each(|i|{
                if bits[i] && now > a[i] { b[i] = now - a[i] + 1; }
                now = std::cmp::max(now, a[i] + b[i])
            });
            let sum = b.iter().sum();
            Some(sum)
        } else {None}
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn minimize_test_1() {
        let a = vec![3949, 3774, 3598, 3469, 3424];
        let k = 5;
        assert_eq!(minimize(&a, k), 1541);
    }

    #[test]
    fn minimize_test_2() {
        let a = vec![7,4,2,6,4];
        let k = 3;
        assert_eq!(minimize(&a, k), 7);
    }
}
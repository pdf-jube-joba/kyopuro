use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        _n: usize, k: usize,
        a: [Usize1; k],
    }
    println!("{}", compute(&a))
}

fn compute(a: &[usize]) -> usize {
    let k = a.len();
    debug_assert!(k > 0);

    if k % 2 == 0 {
        return (0..k / 2).map(|i| a[2 * i + 1] - a[2 * i]).sum();
    }

    let k = k / 2;
    // a = [a_0,...,a_{2k}] a.len() == 2k+1

    // sum_l[i \in 0..=k] = if i == 0 then 0 else {\sum_{j = 0}^{i - 1} a_{2j + 1} - a_{2j}}
    let sum_l: Vec<usize> = {
        let mut v = vec![0; k + 1];
        let mut sum = 0;
        for i in 0..k {
            sum += a[2 * i + 1] - a[2 * i];
            v[i + 1] = sum;
        }
        v
    };

    // sum_r[i \in 0..=k] = if i == k then 0 else \sum_{j = i}^{k-1} a_{2j + 2} - a_{2j}
    let sum_r: Vec<usize> = {
        let mut v = vec![0; k + 1];
        let mut sum = 0;
        for i in 0..k {
            let j = k - 1 - i;
            sum += a[2 * j + 2] - a[2 * j + 1];
            v[j] = sum;
        }
        v
    };

    (0..=k).map(|i| sum_l[i] + sum_r[i]).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(compute(&[1, 2]), 1);
        assert_eq!(compute(&[1, 2, 3, 5]), 3);

        assert_eq!(compute(&[1]), 0);
        assert_eq!(compute(&[5]), 0);
        assert_eq!(compute(&[1, 2, 3]), 1);
        assert_eq!(compute(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(compute(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(compute(&[1, 2, 4, 8, 16]), 5);
    }
}

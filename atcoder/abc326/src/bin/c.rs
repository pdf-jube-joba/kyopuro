fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [usize; n],
    }
    println!("{}", max_get(a, m));
}

fn max_get(mut a: Vec<usize>, m: usize) -> usize {
    let n = a.len();
    a.sort();
    let mut r = 0;
    (0..n)
        .map(|l| {
            // r satisfy r < n && a[r] < a[l] + m
            while r < n && (a[r] < a[l] + m) {
                r += 1;
            }
            // r == max { j in 0..n | a[j] < a[l] + m } + 1
            r -= 1;
            // r == max { j in 0..n | a[j] < a[l] + m }
            r - l + 1
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn max_get_test() {
        assert_eq!(max_get(vec![1], 1), 1);
        assert_eq!(max_get(vec![1], 2), 1);
        assert_eq!(max_get(vec![1, 2], 1), 1);
        assert_eq!(max_get(vec![1, 2], 3), 2);
    }
}

fn main() {
    proconio::input! {
        n: usize,
    }
    let min_326_like = (n..).find(|i| is_326_like(*i)).unwrap();
    println!("{}", min_326_like);
}

fn is_326_like(n: usize) -> bool {
    let n_100 = (n / 100) % 10;
    let n_10 = (n / 10) % 10;
    let n_1 = n % 10;
    n_100 * n_10 == n_1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_326() {
        assert!(is_326_like(326));
        assert!(is_326_like(400));
        assert!(is_326_like(144));
        assert!(!is_326_like(623));
        assert!(!is_326_like(777));
        assert!(!is_326_like(429));
    }
}

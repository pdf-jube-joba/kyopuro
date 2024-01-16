use num_traits::Pow;

fn main() {
    proconio::input! {
        b: usize,
    }
    println!(
        "{}",
        if let Some(a) = find_apowa(b) {
            a.to_string()
        } else {
            "-1".to_string()
        }
    )
}

fn find_apowa(b: usize) -> Option<usize> {
    (1..=15)
        .find(|a: &usize| (*a as u128).pow(*a as u32) == b as u128)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(find_apowa(1), Some(1));
        assert_eq!(find_apowa(2), None);
        assert_eq!(find_apowa(4), Some(2));
        for i in 1..16 {
            assert_eq!(find_apowa(i.pow(i as u32)), Some(i));
        }
        assert_eq!(find_apowa(10_usize.pow(18_u32)), None);
    }
}

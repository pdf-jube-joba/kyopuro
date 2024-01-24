use num_integer::Roots;

fn main() {
    proconio::input! {
        n: usize,
        s: usize,
    }
    println!("{}", count_sq(n, s))
}

fn count_sq(n: usize, s: usize) -> usize {
    let max_check = 10_usize.pow(n as u32).sqrt();
    let mut v = get_rev_sorted(s);
    v.resize(n, 0);
    (0..=max_check)
        .filter(|i| {
            let mut v2 = get_rev_sorted(i.pow(2));
            v2.resize(n, 0);
            v == v2
        })
        .count()
}

fn get_rev_sorted(mut n: usize) -> Vec<usize> {
    let mut v = vec![];
    while n != 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.sort();
    v.reverse();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(count_sq(13, 8694027811503), 840);
    }
    // これ間違えてしまった...
    #[test]
    fn small() {
        assert_eq!(count_sq(1, 9), 1)
    }
}

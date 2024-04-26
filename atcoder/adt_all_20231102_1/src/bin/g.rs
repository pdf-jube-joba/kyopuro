fn main() {
    proconio::input! {
        k: usize,
    }
    println!("{}", factorial_and_multiple(k))
}

fn factorial_and_multiple(mut k: usize) -> usize {
    let mut prime_num: Vec<(usize, usize)> = vec![];
    // old(k) = Pi_{(p, a) in prime_num} p^a * now(k)
    for p in (2_usize..).take_while(move |p| p.pow(2_u32) <= k) {
        let mut n = 0;
        while k % p == 0 {
            k /= p;
            n += 1;
        }
        if n > 0 {
            prime_num.push((p, n))
        }
    }
    if k > 1 {
        prime_num.push((k, 1));
    }
    prime_num
        .into_iter()
        .map(|(p, mut a)| f(p, a))
        .max()
        .unwrap()
}

fn f(p: usize, a: usize) -> usize {
    let mut a: isize = a as isize;
    let mut n = 0;
    while a > 0 {
        n += p;
        let mut x = n;
        while x % p == 0 {
            x /= p;
            a -= 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ff() {
        f(2, 5);
    }
}

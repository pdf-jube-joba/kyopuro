use ac_library::ModInt998244353 as ModInt;
use num_integer::lcm;

fn main() {
    proconio::input! {
        n: usize, k: usize,
    }
    println!("{}", score(n, k))
}

fn score(n: usize, k: usize) -> ModInt {
    let mut partition = vec![1; n];
    let mut ans = ModInt::new(0);
    loop {
        let score: usize = partition.iter().fold(1, |acm, y| lcm(acm, *y));
        let count = count_of_loop(&partition);
        // eprintln!("{:?} {:?} {:?}", partition, count, score);
        ans += count * (ModInt::new(score)).pow(k as u64);
        if !next_partition(&mut partition) {
            break;
        }
    }
    ans
}

fn factorial(n: usize) -> ModInt {
    let mut ans = ModInt::new(1);
    for i in 1..=n {
        ans *= i;
    }
    ans
}

fn count_of_loop(partition: &Vec<usize>) -> ModInt {
    let n: usize = partition.iter().sum();
    let lf = {
        let mut v = vec![0; *(partition.first().unwrap()) + 1];
        for l in partition {
            v[*l] += 1;
        }
        v
    };
    factorial(n)
        * lf.into_iter()
            .enumerate()
            .skip(1)
            .map(|(l, f)| factorial(l - 1).pow(f) / (factorial(l).pow(f) * factorial(f as usize)))
            .product::<ModInt>()
}

fn next_partition(v: &mut Vec<usize>) -> bool {
    debug_assert!(!v.is_empty() && v.iter().all(|vi| *vi > 0));
    if v.len() == 1 {
        return false;
    }
    let i = (1..v.len()-1).rev().find(|&i| v[i-1] > v[i]).unwrap_or(0);
    let rem: usize = v[i+1..].iter().sum();
    v.truncate(i+1);
    v[i] += 1;
    v.extend(vec![1; rem - 1]);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        score(6, 1);
    }
}

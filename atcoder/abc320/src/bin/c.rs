use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        m: usize,
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }
    let s1 = s1
        .into_iter()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect_vec();
    let s2 = s2
        .into_iter()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect_vec();
    let s3 = s3
        .into_iter()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect_vec();
    println!(
        "{}",
        if let Some(t) = min_step(s1, s2, s3) {
            t.to_string()
        } else {
            "-1".to_string()
        }
    )
}

fn min_step(s1: Vec<usize>, s2: Vec<usize>, s3: Vec<usize>) -> Option<usize> {
    let m = s1.len();
    (0..10)
        .filter_map(|digit| {
            // if either one of s has no digit
            if s1.iter().all(|d| *d != digit)
                || s2.iter().all(|d| *d != digit)
                || s3.iter().all(|d| *d != digit)
            {
                return None;
            }

            let (mut b1, mut b2, mut b3) = (false, false, false);
            for i in 0.. {
                if !b1 && s1[i % m] == digit {
                    b1 = true;
                } else if !b2 && s2[i % m] == digit {
                    b2 = true;
                } else if !b3 && s3[i % m] == digit {
                    b3 = true;
                }

                if b1 && b2 && b3 {
                    return Some(i);
                }
            }
            unreachable!()
        })
        .min()
}

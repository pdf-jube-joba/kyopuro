use itertools::Itertools;

const M: usize = 30;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    let a: Vec<Vec<bool>> = a
        .into_iter()
        .map(|ai| (0..M).map(|i| ai & (1 << i) != 0).collect())
        .collect();
    println!("{}", xor_minimization(a).unwrap());
}

// a[[bool; M; N] |-> min_{x: [bool; M]} max_{i in 0..N} sum_{k in 0..M} (a[i][k] ^^ x[k]) * 2^k
// if N == 0 return None
fn xor_minimization(a: Vec<Vec<bool>>) -> Option<usize> {
    if a.is_empty() {
        return None;
    }
    let m = a[0].len();
    if m == 0 {
        return Some(0);
    }
    let mut last_t = vec![];
    let mut last_f = vec![];
    for mut ai in a {
        let last = ai.pop().unwrap();
        if last {
            last_t.push(ai);
        } else {
            last_f.push(ai);
        }
    }
    match (xor_minimization(last_t), xor_minimization(last_f)) {
        (Some(m1), Some(m2)) => Some(std::cmp::min(m1, m2) + 2_usize.pow((m - 1) as u32)),
        (Some(m), None) | (None, Some(m)) => Some(m),
        (None, None) => unreachable!(),
    }
}

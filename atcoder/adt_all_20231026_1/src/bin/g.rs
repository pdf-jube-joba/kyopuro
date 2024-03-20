use itertools::Itertools;
use itertools_num::ItertoolsNum;

fn main() {
    proconio::input! {
        n: usize, m: usize, p: usize,
        a: [usize; n],
        b: [usize; m],
    }
    println!("{}", set_menu(p, a, b))
}

fn set_menu(p: usize, mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
    a.sort();
    b.sort();
    let (n, m) = (a.len(), b.len());
    let s = std::iter::once(0).chain(b.iter().cumsum()).collect_vec();
    let mut all_sum = 0;
    for ai in a {
        let l = b.partition_point(|bi| ai + bi <= p);
        // sum_{i in 0..m} min(ai + b[i], p) = sum_{i in 0..l} ai + b[i] + sum_{i in l..m} p
        // = ai * l + s[l] + p * (m - l)
        all_sum += ai * l + s[l] + p * (m - l);
    }
    all_sum
}

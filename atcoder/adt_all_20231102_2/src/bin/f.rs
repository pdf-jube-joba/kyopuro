fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", approximate_equalization2(a))
}

fn approximate_equalization2(mut a: Vec<usize>) -> usize {
    let n = a.len();
    a.sort();
    let asum: usize = a.iter().sum::<usize>();
    let (p, r) = (asum / n, asum % n);
    let b: Vec<usize> = std::iter::repeat(p)
        .take(n - r)
        .chain(std::iter::repeat(p + 1).take(r))
        .collect();
    (0..n).map(|i| a[i].abs_diff(b[i])).sum::<usize>() / 2
}

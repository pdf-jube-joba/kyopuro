fn main() {}

// sum_{i in 0..l} n + l
fn sums2(n: usize, l: usize) -> usize {
    (n + l) * l / 2
}

fn sums(n: usize, l: usize) -> usize {
    (0..l).map(|i| n + i).sum()
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]
fn check_sum() {
    let n: usize = kani::any();
    let l: usize = kani::any();
    kani::assume(l < n && n < 1000);
    let res = sums2(n, l);
    let exp = sums(n, l);
    // assert_eq!(res, exp);
}

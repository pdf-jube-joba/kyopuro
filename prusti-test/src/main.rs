use prusti_contracts::*;

#[requires(n <= 1000 && l <= 1000)]
fn sums(n: usize, l: usize) -> usize {
    (n..n+l).sum()
}

#[requires(n <= 1000 && l <= 1000)]
#[ensures(result == sums(n, l))]
fn sums2(n: usize, l: usize) -> usize {
    (n + l) * l / 2 - (l / 2) 
}

fn main() {}

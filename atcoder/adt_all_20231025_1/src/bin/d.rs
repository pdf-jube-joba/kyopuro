use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut num = vec![0; n];
    for (a, b) in ab {
        num[a] += 1;
        num[b] += 1;
    }
    let b = num.into_iter().any(|numi| numi == n - 1);
    println!("{}", if b { "Yes" } else { "No" })
}

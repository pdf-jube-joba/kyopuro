use itertools::iproduct;

fn main() {
    proconio::input! {
        n: usize,
        d: [usize; n],
    }
    println!("{}", count(d))
}

fn count(d: Vec<usize>) -> usize {
    let n = d.len();
    (1..=n)
        .flat_map(|i| (1..=d[i - 1]).map(move |d| (i, d)))
        .filter(|&(mut m, mut d)| {
            let t = m % 10;
            while m > 0 {
                if m % 10 != t {
                    return false;
                }
                m /= 10;
            }
            while d > 0 {
                if d % 10 != t {
                    return false;
                }
                d /= 10;
            }
            true
        })
        .count()
}

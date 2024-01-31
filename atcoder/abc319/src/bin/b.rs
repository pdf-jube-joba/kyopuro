fn main() {
    proconio::input! {
        n: usize,
    }
    for i in 0..=n {
        print!(
            "{}",
            if let Some(j) = min(n, i) {
                j.to_string()
            } else {
                "-".to_string()
            }
        )
    }
    println!()
}

fn min(n: usize, i: usize) -> Option<usize> {
    (1..=9).find(|j| n % j == 0 && (i % (n / j) == 0))
}

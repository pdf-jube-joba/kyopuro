fn main() {
    proconio::input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    println!("{}", len_from_left(ab))
}

fn len_from_left(ab: Vec<(usize, usize)>) -> f64 {
    let mut t: f64 = ab.iter().map(|&(a, b)| a as f64 / b as f64).sum::<f64>() / 2_f64;
    let mut ans = 0_f64;
    for (a, b) in ab {
        if t < (a as f64 / b as f64) {
            return ans + t * (b as f64);
        } else {
            ans += a as f64;
            t -= a as f64 / b as f64;
        }
    }
    unreachable!()
}

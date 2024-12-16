fn main() {
    proconio::input! {
        p: f64
    }

    println!("{}", best(p));
}

fn best(p: f64) -> f64 {
    let f = |x| x + (p / (2_f64.powf(x / 1.5)));
    let mut l = 0_f64;
    let mut r = p;
    while r - l > 0.000_000_001 {
        let t1 = l + (r - l) / 3.0;
        let t2 = r - (r - l) / 3.0;
        if f(t1) < f(t2) {
            r = t2;
        } else {
            l = t1;
        }
    }
    f(l)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn bext_test_1() {
        let result = (best(3.0) - 2.87089).abs() < 0.001;
        assert!(result)
    }
}

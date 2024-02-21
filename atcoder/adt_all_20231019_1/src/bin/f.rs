fn main() {
    proconio::input! {
        n: usize, k: usize, x: usize,
        a: [usize; n],
    }
    println!("{}", min_cost(a, (k, x)))
}

fn min_cost(mut a: Vec<usize>, (mut k, x): (usize, usize)) -> usize {
    let n = a.len();
    a.sort();
    let mut last = a.len() - 1;
    // spend coupon as much as we can without loss.
    loop {
        let use_num = std::cmp::min(a[last] / x, k);
        a[last] -= use_num * x;
        k -= use_num;
        if last == 0 {
            break;
        } else {
            last -= 1;
        }
    }

    a.sort();

    // spend coupon from highest amont of money
    if n <= k {
        0
    } else {
        a[0..n - k].iter().sum()
    }
}

fn main() {
    proconio::input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    println!("{}", amusement_park(k, a))
}

fn amusement_park(mut k: usize, mut a: Vec<usize>) -> usize {
    a.sort_by_key(|&i| std::cmp::Reverse(i));
    let get = |i: usize| -> usize { a.get(i).cloned().unwrap_or(0) };
    let mut c = 0;
    let mut n = 0;
    loop {
        let i = a.partition_point(|&ai| ai >= a[n]);
        let d = get(i - 1) - get(i);
        if k < d * i {
            c += sum_dec(a[n], k / i) * (i - k % i) + sum_dec(a[n], k / i + 1) * (k % i);
            return c;
        } else {
            k -= d * i;
            c += i * sum_dec(a[n], d);
            if i == a.len() {
                return c;
            }
            n = i;
        }
    }
}

// sum_{i = 0..l} (n - i)
fn sum_dec(n: usize, l: usize) -> usize {
    (2 * n - l + 1) * l / 2
}

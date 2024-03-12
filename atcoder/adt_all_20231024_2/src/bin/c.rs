fn main() {
    proconio::input! {
        a: u64, b: u64,
    }
    println!("{}", if have_inc(a, b) {"Hard"} else {"Easy"})
}

fn b10digit(mut t: u64) -> Vec<usize> {
    let mut v = vec![];
    while t != 0 {
        v.push((t % 10) as usize);
        t /= 10;
    }
    v
}

fn have_inc(a: u64, b: u64) -> bool {
    let a = b10digit(a);
    let b = b10digit(b);
    let n = std::cmp::min(a.len(), b.len());
    (0..n).any(|i| a[i] + b[i] >= 10)
}

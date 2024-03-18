fn main() {
    proconio::input! {
        n: usize, x: isize,
        a: [isize; n],
    }
    println!("{}", if gap_existence(a, x) { "Yes" } else { "No" })
}

fn gap_existence(mut a: Vec<isize>, x: isize) -> bool {
    let n = a.len();
    a.sort();
    (0..n).any(|i| {
        let ai = a[i];
        a[..].binary_search(&(ai - x)).is_ok()
    })
}

use proconio::fastout;

#[fastout]
fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [usize; m],
    }
    let mut c = 0;
    // ci = min { c | a[c] >= i }
    for i in 1..=n {
        println!("{}", a[c] - i);
        if a[c] - i == 0 {
            c += 1;
        }
    }
}

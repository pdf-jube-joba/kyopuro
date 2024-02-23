use num_integer::{div_floor, Roots};

fn main() {
    proconio::input! {
        n: usize,
    }
    let m = n.sqrt();
    println!(
        "{}",
        2 * (1..=m).map(|i| div_floor(n, i)).sum::<usize>() - m.pow(2)
    )
}

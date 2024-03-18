use num_traits::Pow;

fn main() {
    proconio::input! {
        n: isize,
    }
    let b = -(2_isize.pow(31_u32)) <= n && n < 2_isize.pow(31_u32);
    println!("{}", if b { "Yes" } else { "No" })
}

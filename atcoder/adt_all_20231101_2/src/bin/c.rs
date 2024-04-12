use itertools::iproduct;

fn main() {
    proconio::input! {
        n: usize,
        s: [usize; n],
    }
    let n = s.into_iter().filter(|&si|{
        iproduct!(1..=1000, 1..=1000).all(|(a, b)|{
            4 * a * b + 3 * a + 3 * b != si
        })
    }).count();
    println!("{}", n)
}

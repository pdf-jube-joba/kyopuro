use itertools::Itertools;

fn main() {
    proconio::input! {
        k: usize,
    }
    let chars: Vec<char> = ('A'..='Z').collect();
    println!("{}", chars[0..k].iter().join(""))
}

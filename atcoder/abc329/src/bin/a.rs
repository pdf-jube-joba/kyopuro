use itertools::Itertools;

fn main() {
    proconio::input! {
        s: String,
    }
    println!("{}", s.chars().join(" "))
}

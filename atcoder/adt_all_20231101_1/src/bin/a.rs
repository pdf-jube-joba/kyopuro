fn main() {
    proconio::input!{
        y: usize,
    }
    let ny = (y..).find(|&y| y % 4 == 2).unwrap();
    println!("{ny}")
}

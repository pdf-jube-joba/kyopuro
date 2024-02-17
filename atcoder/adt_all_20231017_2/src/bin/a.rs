fn main() {
    proconio::input!{
        k: usize,
    }
    println!("{:02}:{:02}", 21 + k/60, k % 60)
}

fn main() {
    proconio::input! {
        b: usize, g: usize,
    }
    if b < g {
        println!("Glove");
    } else {
        println!("Bat");
    }
}

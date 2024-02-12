fn main() {
    proconio::input! {
        s: String,
    }
    println!("{}", if s == "Hello,World!" { "AC" } else { "WA" })
}

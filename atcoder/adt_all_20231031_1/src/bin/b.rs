fn main() {
    proconio::input! {
        l: usize, r: usize,
    }
    let s = "atcoder";
    println!("{}", &s[l-1..r])
}

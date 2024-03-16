fn main() {
    proconio::input! {
        a: usize, b: usize,
    }
    let b = !(a == 3 || a == 6) && (b - a == 1);
    println!("{}", if b {"Yes"} else {"No"})
}

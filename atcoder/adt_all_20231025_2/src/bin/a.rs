fn main() {
    proconio::input!{
        n: usize,
    }
    let n = if n <= 125 {
        4
    } else if n <= 211 {
        6
    } else {
        8
    };
    println!("{}", n)
}

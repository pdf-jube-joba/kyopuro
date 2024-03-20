fn main() {
    proconio::input!{
        x: usize,        
    }
    println!("{}", if x % 100 == 0 && x >= 100 {"Yes"} else {"No"})
}

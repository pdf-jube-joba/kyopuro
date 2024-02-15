fn main() {
    proconio::input! {
        k: usize,
    }
    let res: String = (0..=63)
        .rev()
        .map(|i| if k & (1 << i) != 0 { "2" } else { "0" })
        .skip_while(|s| *s == "0")
        .collect::<String>();
    println!("{}", res);
}

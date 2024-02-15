fn main() {
    proconio::input! {
        s: String,
    }
    let else_0: usize = s
        .chars()
        .filter(|&b| b.is_ascii_digit() && b != '0')
        .count();
    let count_0: usize = s
        .split(|char: char| char.is_ascii_digit() && char != '0')
        .map(|b| (b.len() + 1) / 2)
        .sum();
    println!("{}", else_0 + count_0);
}

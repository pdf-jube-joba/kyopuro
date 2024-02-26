use num_traits::Pow;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        k: usize,
        a: Chars, b: Chars,
    }
    let convert = |c: Vec<char>| -> usize {
        c.into_iter()
            .rev()
            .enumerate()
            .map(|(i, ai)| ai.to_digit(k as u32).unwrap() as usize * k.pow(i as u32))
            .sum()
    };
    let a: usize = convert(a);
    let b: usize = convert(b);
    println!("{}", a * b)
}

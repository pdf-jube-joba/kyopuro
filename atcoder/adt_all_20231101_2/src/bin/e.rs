use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", secret_number(s));
}

fn secret_number(s: Vec<char>) -> usize {
    (0..=9999)
        .filter(|i| {
            let is = vec![i / 1000, (i / 100) % 10, (i / 10) % 10, i % 10];
            (0..=9).all(|t| match s[t] {
                'o' => is.contains(&t),
                'x' => !is.contains(&t),
                _ => true,
            })
        })
        .count()
}

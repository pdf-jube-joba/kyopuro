use proconio::marker::Chars;

fn main() {
    proconio::input! {
        mut s: Chars,
    }
    s.reverse();
    let id: usize = s
        .into_iter()
        .enumerate()
        .map(|(i, ci)| 26usize.pow(i as u32) * (ci as usize - 'A' as usize + 1))
        .sum::<usize>();
    println!("{id}")
}

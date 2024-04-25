use proconio::marker::Chars;

fn main() {
    proconio::input!{
        mut s: Chars,
    }
    s.sort();
    s.dedup();
    let n = match s.len() {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => unreachable!()
    };
    println!("{n}")
}

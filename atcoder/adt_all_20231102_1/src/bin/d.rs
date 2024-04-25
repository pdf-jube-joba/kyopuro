use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", if sandwich_number(s) { "Yes" } else { "No" })
}

fn sandwich_number(s: Vec<char>) -> bool {
    if s.len() != 8 {
        return false;
    }
    let (first, last) = (s.first().unwrap(), s.last().unwrap());
    if !first.is_ascii_uppercase() || !last.is_ascii_uppercase() {
        return false;
    }
    let s = s[1..7].to_owned();
    if s.iter().any(|c| !c.is_numeric()) {
        return false;
    }
    if s[0] == '0' {
        return false;
    }
    true
}

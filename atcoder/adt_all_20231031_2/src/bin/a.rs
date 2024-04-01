use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let c = if s[0] != s[1] && s[0] != s[2] {
        Some(s[0])
    } else if s[1] != s[0] && s[1] != s[2] {
        Some(s[1])
    } else if s[2] != s[0] && s[2] != s[1] {
        Some(s[2])
    } else {
        None
    };
    println!(
        "{}",
        if let Some(c) = c {
            c.to_string()
        } else {
            "-1".to_string()
        }
    )
}

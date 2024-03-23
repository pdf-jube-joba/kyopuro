use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
    }
    let ans = longest_uncommon_prefic(s);
    for ans in ans {
        println!("{ans}");
    }
}

// s[0..n] for i in 1..n, max { l in 0..=n-i | s[k] != s[k+i] forall k in 0..l }
fn longest_uncommon_prefic(s: Vec<char>) -> Vec<usize> {
    let n = s.len();
    (1..n)
        .map(|i| {
            let mut l = 1;
            while l <= n - i {
                debug_assert!((0..(l - 1)).all(|k| s[k] != s[k + i]));
                if s[l - 1] != s[l - 1 + i] {
                    l += 1;
                } else {
                    break;
                }
            }
            l - 1
        })
        .collect()
}

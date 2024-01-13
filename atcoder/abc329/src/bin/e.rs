use std::collections::VecDeque;

use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars,
    }
    println!("{}", if stamp(s, t) { "Yes" } else { "No" })
}

fn stampable(s: &[char], i: usize, t: &[char]) -> bool {
    let m = t.len();
    debug_assert!(i + m <= s.len());
    (0..m).all(|j| {
        s[i + j] == t[j] || s[i + j] == '#'
    })
}

fn stamp(mut s: Vec<char>, t: Vec<char>) -> bool {
    let (n, m) = (s.len(), t.len());
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut stamped = vec![false; n];

    queue.extend((0..=n - m).filter(|i| stampable(&s[..], *i, &t[..])));

    while let Some(ind) = queue.pop_front() {
        #[cfg(test)]
        eprintln!("{} {:?} {:?}", ind, queue, s);
        if stamped[ind] {
            continue;
        }
        stamped[ind] = true;

        for i in 0..m {
            s[ind + i] = '#';
        }

        // \{ i \in [0..n-m+1) \mid [i..i+m) \cap [ind..ind+m) \not = \emptyset \} = \{ i \in [0.. n-m+1) \mid ind < i+m && i < ind+m \} = [max(ind + 1-m,0), min(n-m+1, ind+m))
        let l = (ind + 1).saturating_sub(m);
        let r = std::cmp::min(n - m + 1, ind + m);
        let n = (l..r)
            .filter(|i| stampable(&s[..], *i, &t[..]))
            .collect_vec();
        #[cfg(test)]
        eprintln!("[{}, {}) {:?}", l, r, n);
        queue.extend(n);
    }

    s.into_iter().all(|char| char == '#')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stamp_test() {
        let tests_succ = vec![
            // 
            ("a", "a"),
            ("aaa", "a"),
            ("aaab", "ab"),
            ("abbb", "ab"),
            ("abab", "ab"),
            ("ababa", "aba"),
        ];
        let tests_fail = vec![
            // 
            ("bb", "ab"),
            ("ba", "ab"),
            ("bab", "ab"),
            ("ababa", "ab"),
        ];
        for (s, t) in tests_succ {
            assert!(stamp(s.chars().collect_vec(), t.chars().collect_vec()));
        }
        for (s, t) in tests_fail {
            assert!(!stamp(s.chars().collect_vec(), t.chars().collect_vec()))
        }
    }
}

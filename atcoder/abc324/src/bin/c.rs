use itertools::{enumerate, Itertools};
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, t_: Chars,
        s: [Chars; n],
    }
    let possible = s
        .into_iter()
        .enumerate()
        .filter_map(|(i, si)| {
            if maybe_changed(&t_, &si) {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect_vec();
    println!("{}\n{}", possible.len(), possible.into_iter().join(" "))
}

fn maybe_changed(t1: &Vec<char>, t2: &Vec<char>) -> bool {
    let n1 = t1.len();
    let n2 = t2.len();
    if n1.abs_diff(n2) > 1 {
        return false;
    }
    // |n1 - n2| <= 1

    // p = max { j in 0..min(n1, n2) | t1[0..j] == t2[0..j] }
    let p: usize = {
        let prefix_i = (0..std::cmp::min(n1, n2)).find(|i| t1[*i] != t2[*i]);
        match prefix_i {
            Some(i) => i,
            None => {
                // |n1 - n2| <= 1
                // n1 == n2 => t1 == t2, n1 + 1 == n2 => remove last char, n2 = n1 + 1 => insert at last
                return true;
            }
        }
    };

    // s = max { j in 0..min(n1, n2) | t1[n1-j..n1] == t2[n2-j..n2] <=> rev(t1)[0..j] == rev(t2)[0..j] }
    let s: usize = {
        // i in 0..suffix_i == (t1[n1-i-1] == t2[n2-i-1]) => t1[n1-suffix_i..n1] == t2[n2-suffix_i..n2]
        let suffix_i = (0..std::cmp::min(n1, n2)).find(|i| t1[n1 - 1 - *i] != t2[n2 - 1 - *i]);
        match suffix_i {
            Some(i) => i,
            None => {
                // |n1 - n2| <= 1
                // n1 == n2 => t1 == t2 (but never happen because it is checkced at prefix)
                // n1 + 1 == n2 => remove first char, n2 == n1 + 1 => insert at first
                debug_assert!(n1 != n2);
                return true;
            }
        }
    };

    // we can assume t1[prefix_i] != t2[prefix_i] && t1[n1 - suffix_i] != t2[n2 - suffix_i]

    // change case
    if n1 == n2 {
        debug_assert!(p <= n1 - s - 1);
        p == n1 - s - 1
    // insert case
    } else if n1 == n2 + 1 {
        p >= n1 - s - 1
    // remove case
    } else if n2 == n1 + 1 {
        p >= n2 - s - 1
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch() {
        assert!(maybe_changed(
            &"abc".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));
        assert!(maybe_changed(
            &"abc".chars().collect_vec(),
            &"abbc".chars().collect_vec()
        ));
        assert!(maybe_changed(
            &"aba".chars().collect_vec(),
            &"aa".chars().collect_vec()
        ));
        assert!(maybe_changed(
            &"aba".chars().collect_vec(),
            &"aca".chars().collect_vec()
        ));

        assert!(maybe_changed(
            &"aba".chars().collect_vec(),
            &"aca".chars().collect_vec()
        ));

        assert!(maybe_changed(
            &"aa".chars().collect_vec(),
            &"aab".chars().collect_vec()
        ));

        assert!(!maybe_changed(
            &"aab".chars().collect_vec(),
            &"aba".chars().collect_vec()
        ));
        assert!(!maybe_changed(
            &"a".chars().collect_vec(),
            &"aaa".chars().collect_vec()
        ));
    }
}

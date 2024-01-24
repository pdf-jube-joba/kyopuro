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
    debug_assert!(n1 > 0 && n2 > 0);
    if n1.abs_diff(n2) > 1 {
        return false;
    }

    // t1[0..prefix_i] == t2[0..prefix_i]
    let prefix_i: usize = {
        let prefix_i = (0..std::cmp::min(n1, n2)).find(|i| t1[*i] != t2[*i]);
        match prefix_i {
            Some(i) => i,
            None => {
                return true;
            }
        }
    };

    // t1[n1-suffix_i..n1] == t2[n2-suffix_i..n2]
    let suffix_i: usize = {
        let suffix_i = (0..std::cmp::min(n1, n2)).find(|i| t1[n1 - *i - 1] != t2[n2 - *i - 1]);
        match suffix_i {
            Some(i) => i,
            None => {
                return true;
            }
        }
    };

    (prefix_i == suffix_i && suffix_i == n1 && n1 == n2) // t1 == t2
    || prefix_i + suffix_i >= n1 && n1 == n2 - 1 // insert some char to t1 == t2
    || prefix_i + suffix_i >= n2 - 1 && n1 - 1 == n2 // remove some char from t1 == t2
    || prefix_i + suffix_i == n1 - 1 && n1 - 1 == n2 - 1 // change some char in t1 == t2

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch() {
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
        // assert!(maybe_changed(
        //     &"".chars().collect_vec(),
        //     &"".chars().collect_vec()
        // ));
        // assert!(maybe_changed(
        //     &"".chars().collect_vec(),
        //     &"".chars().collect_vec()
        // ));
    }
}

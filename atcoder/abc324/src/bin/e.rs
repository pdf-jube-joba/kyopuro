use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, t: Chars,
        s: [Chars; n],
    }
    println!("{}", num(t, s))
}

fn num(t: Vec<char>, s: Vec<Vec<char>>) -> usize {
    let n = s.len();
    // subsuf_num[l] = #{ si | si contains t[t.len()-l..t.len()] as noncontinuous substring and l is max of such l }
    let mut subsuf_num: Vec<usize> = vec![0; t.len() + 1];
    for si in &s {
        let subsuf = subsuf_contain(&t, si);
        subsuf_num[subsuf.len()] += 1;
    }

    // for each k, it holds
    // # {(si, sj) |
    //  - si contains t[0..k] as (non continuous) subtring
    //  - and k is max of such k
    //  - sj contains t[k..] as (non continuous) substring
    // }
    // == subpre_num[k] + \sum_{k + l >= t.len()} subpre_num[l]

    (0..s.len())
        .map(|i| {
            let pre_i = subpre_contain(&t, &s[i]).len();
            (t.len() - pre_i..=t.len())
                .map(|l| subsuf_num[l])
                .sum::<usize>()
        })
        .sum()

    // (0..=t.len())
    //     .map(|k| {
    //         (t.len() - k..=t.len())
    //             .map(|l| subpre_num[k] * subsuf_num[l])
    //             .sum::<usize>()
    //     })
    //     .sum()
}

// return a[0..k] s.t. s[a[i]] = t[i] && a[i] < a[i+1]
// && has max length of such a[0..k]
fn subpre_contain(t: &[char], s: &[char]) -> Vec<usize> {
    let mut v = vec![];
    let mut k = 0;
    for i in 0..s.len() {
        if s[i] == t[k] {
            v.push(i);
            if k == t.len() - 1 {
                break;
            } else {
                k += 1;
            }
        }
    }
    v
}

// return a[0..k] s.t. s[a[i]] = t[s.len() -i+1] && a[i] > a[i+1]
// && has max length of such a[0..k]
fn subsuf_contain(t: &[char], s: &[char]) -> Vec<usize> {
    let mut v = vec![];
    let mut k = t.len() - 1;
    for i in (0..s.len()).rev() {
        if s[i] == t[k] {
            v.push(i);
            if k == 0 {
                break;
            } else {
                k -= 1;
            }
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn subsuf_test() {
        let tests = vec![("ms", "hlms", vec![3, 2])];
        for (t, s, res) in tests {
            assert_eq!(
                subsuf_contain(&t.chars().collect_vec(), &s.chars().collect_vec()),
                res
            )
        }
    }
}

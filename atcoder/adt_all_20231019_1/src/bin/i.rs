use ac_library::ModInt1000000007 as ModInt;
use im_rc::HashSet;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let dp = num_subst_1skip_set(s);
    eprintln!("{dp:?}");
    println!("{}", dp[1..dp.len()].iter().map(|v| v.len()).sum::<usize>());
    // println!("{}", num_subst_1skip(s))
}

fn num_subst_1skip(s: Vec<char>) -> ModInt {
    let n = s.len();
    // dp[i in 0..=n] = num of 1 skip substr s.t. uses s[i-1]
    let mut dp: Vec<ModInt> = vec![ModInt::new(0); n + 1];
    dp[0] = ModInt::new(1);
    dp[1] = ModInt::new(1);
    // dp[i+1] = sum_{k+1 <= j < i-1} dp[j] where k = max { 0<= k < i | s[i] == s[k] } orelse
    for i in 1..n {
        let k = (0..i)
            .rev()
            .find(|&j| s[i] == s[j])
            .map(|k| k + 1)
            .unwrap_or(0);
        // eprintln!("{i} {k}");
        dp[i + 1] = dp[k..i].iter().sum();
    }
    // eprintln!("{:?}", dp);
    (1..=n).map(|i| dp[i]).sum()
}

fn num_subst_1skip_set(s: Vec<char>) -> Vec<HashSet<Vec<char>>> {
    eprintln!("{s:?}");
    let n = s.len();
    // dp[i in 0..=n] = num of 1 skip substr s.t. uses s[i-1]
    let mut dp: Vec<HashSet<Vec<char>>> = vec![HashSet::new(); n + 1];
    dp[0].insert(vec![]);
    dp[1].insert(vec![s[0]]);
    // dp[i+1] = sum_{k <= j < i-1} dp[j] where k = max { 0<= k < i - 1 | s[i] == s[k] } + 1 orelse 0
    for i in 1..n {
        // do dp[i+1]
        let k = (0..i)
            .rev()
            .find(|&j| s[i] == s[j])
            .map(|k| k + 1)
            .unwrap_or(0);
        eprintln!("{i} {k}");
        // last == s[k] から last == s[i-2] までを使う。 => dp[k+1] から dp[i-1] までを足す。
        for j in k..i {
            let new_set: HashSet<Vec<char>> = dp[j]
                .iter()
                .map(|substr| {
                    let mut substr = substr.clone();
                    substr.push(s[i]);
                    substr
                })
                .collect::<HashSet<_>>();
            dp[i + 1].extend(new_set);
        }
    }
    dp
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn nus() {
        let dp = num_subst_1skip_set("abc".chars().collect_vec());
        println!("{dp:?}");
        // let dp = num_subst_1skip_set("abca".chars().collect_vec());
        // println!("{dp:?}");
        // let dp = num_subst_1skip_set("abcb".chars().collect_vec());
        // println!("{dp:?}");
        // let dp = num_subst_1skip_set("abcc".chars().collect_vec());
        // println!("{dp:?}");
        // let dp = num_subst_1skip_set("abcd".chars().collect_vec());
        // println!("{dp:?}");
    }
    #[test]
    fn num() {
        let all = |v: Vec<HashSet<Vec<char>>>| -> Vec<String> {
            let mut v: Vec<String> = v.into_iter()
                .flat_map(|set| {
                    set.into_iter()
                        .map(|vec| vec.into_iter().collect::<String>())
                })
                .collect();
            v.sort();
            v
        };

        let dp = num_subst_1skip_set("aa".chars().collect_vec());
        eprintln!("{dp:?}");
        assert_eq!(all(dp), vec!["", "a"]);

        let dp = num_subst_1skip_set("aaa".chars().collect_vec());
        eprintln!("{dp:?}");
        assert_eq!(all(dp), vec!["", "a", "aa"]);
    }
}

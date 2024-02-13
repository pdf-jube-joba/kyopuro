use itertools::{iproduct, Itertools};

const LEN: usize = 16;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: [String; n],
        t: [String; m],
    }
    println!(
        "{}",
        if let Some(str) = possible(s, t) {
            str
        } else {
            "-1".to_string()
        }
    )
}

fn gen_str(s: Vec<String>) -> impl Iterator<Item = String> {
    let n = s.len();
    let sum_s: usize = s.iter().map(|si| si.len()).sum();
    let underscore_num = LEN - sum_s - (n - 1);
    iproduct!(
        (0..n + underscore_num - 1).combinations(underscore_num), // how to insert `_`
        (0..n).permutations(n)                                    // order of s
    )
    .map(move |(insert, order)| {
        let mut new_s: Vec<String> = vec![];
        let mut si = 0;
        let mut su = 0;
        for i in 0..n + underscore_num {
            if su < underscore_num && insert[su] == i {
                new_s.push("_".to_owned());
                su += 1;
            } else {
                if si < n {
                    new_s.push(format!("_{}", s[order[si]]));
                }
                si += 1;
            }
        }
        assert!(su == underscore_num && si == n);
        // eprintln!("{:?}", new_s);
        new_s
            .into_iter()
            .collect::<String>()
            .chars()
            .skip_while(|&char| char == '_')
            .collect::<String>()
    })
}

fn possible(s: Vec<String>, mut t: Vec<String>) -> Option<String> {
    t.sort();
    gen_str(s).find(|si| t.binary_search(si).is_err() && 3 <= si.len() && si.len() <= 16)
}

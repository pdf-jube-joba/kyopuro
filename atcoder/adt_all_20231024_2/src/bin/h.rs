use std::collections::HashSet;

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, q: usize,
    }
    let qs = (0..q)
        .map(|_| {
            proconio::input! {
                t: usize,
            }
            if t == 1 {
                proconio::input! {
                    u: Usize1, v: Usize1,
                }
                Q::Con(u, v)
            } else {
                proconio::input! {
                    u: Usize1,
                }
                Q::Del(u)
            }
        })
        .collect_vec();
    let ans = isolation(n, qs);
    println!("{}", ans.iter().join("\n"))
}

#[derive(Debug, Clone)]
enum Q {
    Con(usize, usize),
    Del(usize),
}

fn isolation(n: usize, qs: Vec<Q>) -> Vec<usize> {
    let mut sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut isolation_set: HashSet<usize> = HashSet::from_iter(0..n);
    let mut ans = vec![];

    for q in qs {
        // eprintln!("q:{q:?}");
        match q {
            Q::Con(u, v) => {
                sets[u].insert(v);
                sets[v].insert(u);
                isolation_set.remove(&u);
                isolation_set.remove(&v);
            }
            Q::Del(u) => {
                for v in sets[u].drain().collect_vec() {
                    sets[v].remove(&u);
                    if sets[v].is_empty() {
                        isolation_set.insert(v);
                    }
                }
                isolation_set.insert(u);
            }
        }
        // eprintln!("a: {sets:?} {isolation_set:?}");
        ans.push(isolation_set.len());
    }
    ans
}

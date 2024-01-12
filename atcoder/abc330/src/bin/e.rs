use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, q: usize,
        a: [usize; n],
        ix: [(Usize1, usize); q],
    }
    for elm in mexes(a, ix) {
        println!("{}", elm);
    }
}

fn mexes(mut a: Vec<usize>, ix: Vec<(usize, usize)>) -> Vec<usize> {
    let n = a.len();
    let truncate = |ai: usize| -> usize { std::cmp::min(ai, n + 1) };
    // count[i] = #{ j | min(a[j], n+1) == i }
    let mut count = vec![0; n + 2];
    for ai in &a {
        count[truncate(*ai)] += 1;
    }
    // set = { i | count[i] == 0 }
    // set is not empty
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for (i, count) in count.iter().enumerate() {
        if *count == 0 {
            set.insert(i);
        }
    }
    // count[x] == 0 <=> set.contains(&x)
    ix.into_iter()
        .map(|(i, x)| {
            // remove a[i] from count
            let ai = truncate(a[i]);
            count[ai] -= 1;
            if count[ai] == 0 {
                set.insert(ai);
            }

            a[i] = x;

            let ai = truncate(a[i]);
            if count[ai] == 0 {
                set.remove(&ai);
            }
            count[ai] += 1;

            // minimum of set
            *set.first().unwrap()
        })
        .collect_vec()
}

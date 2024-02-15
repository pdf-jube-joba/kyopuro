use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::fastout;

#[fastout]
fn main() {
    proconio::input! {
        n: usize,
        _a1: usize,
        // write a[i].0 -> a[i].l and a[i].1 -> a[i].r
        a: [(usize, usize); (n - 1)/2],
        q: usize,
        lr: [(usize, usize); q],
    }
    println!(
        "{}",
        ans(a, lr).into_iter().map(|ans| ans.to_string()).join("\n")
    )
}

fn ans(a: Vec<(usize, usize)>, qs: Vec<(usize, usize)>) -> Vec<usize> {
    // s[i] = sum_{j in 0..i} (a[j].r - a[j].l)
    let s: Vec<usize> = std::iter::once(0)
        .chain(a.iter().map(|&(l, r)| r - l).cumsum())
        .collect();
    let f = |x: usize| -> usize {
        let pl = a.partition_point(|&(l, r)| l <= x);
        let pr = a.partition_point(|&(l, r)| r <= x);
        // a[pl-1].l <= x < a[pl].l && a[pr-1].r <= x < a[pr].r
        // => 0 <= pl - pr <= 1

        if pl == pr {
            // case:  <=> a[p-1].r <= x < a[p].l where p == pl == pr
            s[pr] // = sum_{j in 0..p} (a[j].r - a[j].l)
        } else {
            // case: <=> a[p-1].l <= x < a[p-1].r where p == pl, p - 1 = pr
            s[pr] + x - a[pr].0
        }
    };

    qs.into_iter().map(|(l, r)| f(r) - f(l)).collect()
}

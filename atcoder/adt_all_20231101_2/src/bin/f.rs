use im_rc::HashSet;
use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        a: [Usize1; n],
    }
    let ans = find_it(a);
    println!("{}", ans.len());
    println!("{}", ans.into_iter().map(|i| i + 1).join(" "))
}

fn find_it(a: Vec<usize>) -> Vec<usize> {
    let mut aa: HashSet<usize> = HashSet::new();
    let mut now = 0;
    while !aa.contains(&now) {
        aa.insert(now);
        now = a[now];
    }

    let goal = now;
    let mut now = a[now];
    let mut ans = vec![now];
    while now != goal {
        now = a[now];
        ans.push(now);
    }
    ans
}

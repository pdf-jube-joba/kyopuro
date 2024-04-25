fn main() {
    proconio::input! {
        n: usize, k: usize,
        p: [[usize; 3]; n],
    }
    let ans = final_day(k, p);
    for ans in ans {
        println!("{}", if ans { "Yes" } else { "No" })
    }
}

fn final_day(k: usize, p: Vec<Vec<usize>>) -> Vec<bool> {
    let n = p.len();
    let mut now_sums: Vec<usize> = p.iter().map(|pi| pi.iter().sum()).collect();
    now_sums.sort_by_key(|s| std::cmp::Reverse(*s));
    (0..n)
        .map(|i| {
            let full: usize = p[i].iter().sum::<usize>() + 300;
            let highs = now_sums.partition_point(|pi| *pi > full);
            highs + 1 <= k
        })
        .collect()
}

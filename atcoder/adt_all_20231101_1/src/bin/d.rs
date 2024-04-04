use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut a: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        proconio::input! {
            c: usize,
            aa: [usize; c],
        }
        a.push(aa);
    }
    proconio::input! {
        x: usize,
    }
    let ans = roulette(a, x);
    println!("{}", ans.len());
    println!("{}", ans.into_iter().map(|i| i + 1).join(" "))
}

fn roulette(a: Vec<Vec<usize>>, x: usize) -> Vec<usize> {
    let n = a.len();
    let min_num = a
        .iter()
        .filter_map(|ai| {
            if ai.contains(&x) {
                Some(ai.len())
            } else {
                None
            }
        })
        .min();
    let Some(min_num) = min_num else {
        return vec![];
    };
    (0..n)
        .filter(|&i| a[i].contains(&x) && a[i].len() == min_num)
        .collect()
}

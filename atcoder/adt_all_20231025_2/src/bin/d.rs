use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        a: [Usize1; n],
    }
    let ans = call_the_id_number(a);
    println!("{}", ans.len());
    println!("{}", ans.into_iter().map(|i| i + 1).join(" "));
}

fn call_the_id_number(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut now = i;
        while !visited[now] {
            now = a[now];
            visited[now] = true;
        }
    }
    (0..n).filter(|&i| !visited[i]).collect()
}

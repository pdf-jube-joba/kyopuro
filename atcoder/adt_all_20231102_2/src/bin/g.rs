use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }
    println!(
        "{}",
        takahashi_tour(n, ab).into_iter().map(|i| i + 1).join(" ")
    )
}

fn takahashi_tour(n: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let e = {
        let mut e = vec![vec![]; n];
        for (a, b) in ab {
            e[a].push(b);
            e[b].push(a);
        }
        for e in &mut e {
            e.sort();
        }
        e
    };
    let mut visited = vec![false; n];
    let mut ans = vec![];
    dfs(&e, 0, &mut visited, &mut ans);
    ans
}

fn dfs(e: &Vec<Vec<usize>>, now: usize, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    ans.push(now);
    visited[now] = true;
    for &next in &e[now] {
        if !visited[next] {
            dfs(e, next, visited, ans);
            ans.push(now);
        }
    }
}

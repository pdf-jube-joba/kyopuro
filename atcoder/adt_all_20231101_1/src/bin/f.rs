use std::collections::VecDeque;

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }
    println!("{}", dont_be_cycle(n, ab));
}

fn dont_be_cycle(n: usize, ab: Vec<(usize, usize)>) -> usize {
    let m = ab.len();
    let edge: Vec<Vec<usize>> = {
        let mut e = vec![vec![]; n];
        for (a, b) in ab {
            e[a].push(b);
            e[b].push(a);
        }
        e
    };
    let mut conn: Vec<Vec<usize>> = vec![];
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut conn_this = vec![];
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(i);
        while let Some(n) = queue.pop_front() {
            if visited[n] {
                continue;
            }
            conn_this.push(n);
            visited[n] = true;
            for &t in &edge[n] {
                queue.push_back(t);
            }
        }
        conn.push(conn_this);
    }
    m - conn.into_iter().map(|comp| comp.len() - 1).sum::<usize>()
}

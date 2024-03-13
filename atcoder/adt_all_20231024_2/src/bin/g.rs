use std::collections::VecDeque;

use itertools::iproduct;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n1: usize, n2: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }
    println!("{}", min_dist_bridge((n1, n2), ab))
}

fn edgeconv(vertex: usize, edge: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; vertex];
    for (a, b) in edge {
        if a == b {
            continue;
        }
        e[a].push(b);
        e[b].push(a);
    }
    e
}

fn min_dist_bridge((n1, n2): (usize, usize), edge: Vec<(usize, usize)>) -> usize {
    let n = n1 + n2;
    let edge = edgeconv(n, edge);
    let dist1 = min_dist(n, &edge, 0);
    let dist2 = min_dist(n, &edge, n1 + n2 - 1);
    let ms = (0..n1).map(|u| dist1[u].unwrap()).max().unwrap();
    let me = (n1..n).map(|v| dist2[v].unwrap()).max().unwrap();
    ms + me + 1
}

fn min_dist(vertex: usize, edge: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
    let mut dist: Vec<Option<usize>> = vec![None; vertex];
    let mut queue: VecDeque<usize> = VecDeque::new();
    dist[start] = Some(0);
    queue.push_back(start);

    while let Some(next) = queue.pop_front() {
        for &n in &edge[next] {
            if dist[n].is_none() {
                dist[n] = Some(dist[next].unwrap() + 1);
                queue.push_back(n);
            }
        }
    }

    dist
}

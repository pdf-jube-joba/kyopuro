use itertools::Itertools;
use proconio::{fastout, marker::Usize1};

#[fastout]
fn main() {
    proconio::input! {
        n: usize, q: usize,
        x: [usize; n],
        ab: [(Usize1, Usize1); n-1],
        vk: [(Usize1, Usize1); q],
    }
    let k = *vk.iter().map(|(v, k)| k).max().unwrap() + 1;
    let ans_vec = construct_k_vecs(n, ab, x, k);
    for (v, k) in vk {
        println!("{}", ans_vec[v][k]);
    }
}

fn construct_k_vecs(
    n: usize,
    ab: Vec<(usize, usize)>,
    x: Vec<usize>,
    k: usize,
) -> Vec<Vec<usize>> {
    // construct ordered graph
    let edges: Vec<Vec<usize>> = {
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        for &(a, b) in &ab {
            edges[a].push(b);
            edges[b].push(a);
        }

        let mut stack = vec![(0, None)];
        while let Some((next, parent)) = stack.pop() {
            edges[next].retain(|c| Some(*c) != parent);
            for &child in &edges[next] {
                stack.push((child, Some(next)));
            }
        }
        edges
    };

    let mut ans = vec![vec![]; n];

    dfs(&edges, &x, k, 0, &mut ans);

    ans
}

fn dfs(
    edges: &Vec<Vec<usize>>,
    x: &Vec<usize>,
    k: usize,
    vertex: usize,
    ans: &mut Vec<Vec<usize>>,
) {
    for &child in &edges[vertex] {
        dfs(edges, x, k, child, ans)
    }

    let mut merged = edges[vertex]
        .iter()
        .flat_map(|&child| ans[child].clone())
        .collect_vec();
    merged.push(x[vertex]);

    merged.sort();
    merged.reverse();
    merged.truncate(k);

    ans[vertex] = merged;
}

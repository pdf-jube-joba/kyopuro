use std::vec;

fn main() {
    proconio::input! {
        n: usize, q:usize,
        ab: [(usize, usize); n-1],
        px: [(usize, usize); q],
    }

    let result = solve(n, &ab, &px);
//    (0..n).for_each(|i|{
//        print!("{} ", result[i]);
//    });
}
struct TreeGraph(Vec<Vec<usize>>);

impl TreeGraph {
    fn new(n: usize, ab: &[(usize, usize)]) -> TreeGraph {
        let mut graph = vec![Vec::new(); n + 1];
        for &(a, b) in ab {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut parent = vec![None; n + 1];
        parent[1] = Some(0);
        let mut stack: Vec<usize> = vec![1];
        while let Some(now) = stack.pop() {
            for &l in &graph[now] {
                if parent[l].is_none() {
                    parent[l] = Some(now);
                    stack.push(l);
                }
            }
        }
        let mut tree = vec![vec![]; n + 1];
        for i in 0..=n {
            tree[i] = graph[i].iter().filter_map(|x|{
                if Some(*x) != parent[i] {Some(*x)} else {None}
            }).collect();
        }
        TreeGraph(tree)
    }
}

fn dfs(now: usize, tree_ref: &TreeGraph, p: &[usize], dp: &mut [usize], acc: usize, count: usize) {
//    print!("-{}-", count);
    let TreeGraph(ref tree) = tree_ref;
    let next = p[now] + acc;
    dp[now] = next;
    for &node in &tree[now] {
        dfs(node, tree_ref, p, dp, next, count + 1);
    }
}

fn solve(n: usize, ab: &[(usize, usize)], px: &[(usize, usize)]) -> Vec<usize> {
    let p = {
        let mut vec = vec![0; 1 + n];
        for &(a, b) in px {
            vec[a] += b;
        }
        vec
    };
    let tree = TreeGraph::new(n, ab);
    let mut dp = vec![0; n + 1];
    dfs(1,&tree, &p, &mut dp, 0, 0);
    dp
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_1(){
        let n = 4;
        let _q = 3;
        let ab = vec![(1,2),(2,3),(2,4)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result[1..], vec![100,110,111,110]);
    }
    #[test]
    fn test_2(){
        let n = 3;
        let _q = 3;
        let ab = vec![(1,3),(2,3)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result[1..], vec![100,111,101]);
    }
}
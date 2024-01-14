use std::collections::HashMap;

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize, k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }
    println!("{}", min_cost3(n, uvw, k));
}

#[derive(Debug, Clone, PartialEq)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect_vec(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, mut elm: usize) -> usize {
        while elm != self.parent[elm] {
            self.parent[elm] = self.parent[self.parent[elm]];
            elm = self.parent[elm];
        }
        elm
    }
    fn unite(&mut self, mut elm1: usize, mut elm2: usize) -> bool {
        elm1 = self.find(elm1);
        elm2 = self.find(elm2);
        if elm1 == elm2 {
            return false;
        }
        if self.size[elm1] <= self.size[elm2] {
            std::mem::swap(&mut elm1, &mut elm2);
        }
        self.size[elm1] += self.size[elm2];
        self.parent[elm2] = elm1;
        true
    }
    fn all_vertex_is_united(&self) -> bool {
        let mut i = 0;
        while i != self.parent[i] {
            i = self.parent[i];
        }
        self.size[i] == self.parent.len()
    }
}

fn min_cost(n: usize, uvw: Vec<(usize, usize, usize)>, k: usize) -> usize {
    // enumerate maybe_edge = { (i in 0..n, j in 0..n) in edge | i != j }
    uvw.into_iter()
        // enumerate { [t_0, ..., t_{n-1}] | t_i in maybe_edge && t_i != t_j }
        .combinations(n - 1)
        .filter_map(|edges| {
            let mut cost_sum = 0;
            let mut union_find = UnionFind::new(n);
            for (v0, v1, cost) in edges {
                if !union_find.unite(v0, v1) {
                    return None;
                }
                cost_sum += cost;
            }
            if union_find.all_vertex_is_united() {
                Some(cost_sum % k)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn min_cost2(n: usize, uvw: Vec<(usize, usize, usize)>, k: usize) -> usize {
    let uvw: HashMap<(usize, usize), usize> =
        uvw.into_iter().map(|(u, v, w)| ((u, v), w)).collect();
    (0..n)
        .combinations(2)
        .combinations(n - 1)
        .filter_map(|v| {
            let mut cost_sum = 0;
            let mut union_find = UnionFind::new(n);
            for edge in v {
                let [v0, v1, ..] = edge[..] else {
                    unreachable!()
                };
                if !union_find.unite(v0, v1) {
                    return None;
                }
                cost_sum += *uvw.get(&(v0, v1))?;
            }

            if union_find.all_vertex_is_united() {
                Some(cost_sum % k)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn min_cost3(n: usize, uvw: Vec<(usize, usize, usize)>, k: usize) -> usize {
    let costs: Vec<Vec<Option<usize>>> = {
        let mut costs = vec![vec![None; n]; n];
        for (u, v, w) in uvw {
            costs[u][v] = Some(w);
        }
        costs
    };
    (0..n)
        .combinations(2)
        .combinations(n - 1)
        .filter_map(|v| {
            let mut cost_sum = 0;
            let mut union_find = UnionFind::new(n);
            for edge in v {
                let [v0, v1, ..] = edge[..] else {
                    unreachable!()
                };
                if !union_find.unite(v0, v1) {
                    return None;
                }
                cost_sum += costs[v0][v1]?;
            }

            if union_find.all_vertex_is_united() {
                Some(cost_sum % k)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn union_find_test() {
        let mut union_find = UnionFind::new(3);
        assert!(union_find.unite(0, 1));
        assert!(!union_find.unite(0, 1));
        assert_eq!(union_find.find(0), union_find.find(1));
    }
    #[test]
    fn min_cost_test() {
        let n = 3;
        let uvw = vec![(0, 1, 8), (1, 2, 9)];
        let k = 5;
        assert_eq!(min_cost2(n, uvw, k), 2)
    }
}

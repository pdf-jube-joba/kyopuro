use std::{collections::VecDeque, vec};

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }
    let edges_list: Vec<(usize, usize)> = a.into_iter().zip(b.into_iter()).collect();
    let graph = AdjacencyList::from_unordered_edgelist(n, edges_list);
    println!(
        "{}",
        if graph.bipartite().is_some() {
            "Yes"
        } else {
            "No"
        }
    )
}

struct AdjacencyList {
    vertex_num: usize,
    edges: Vec<Vec<usize>>,
}

impl AdjacencyList {
    fn from_unordered_edgelist(vertex_num: usize, edges_list: Vec<(usize, usize)>) -> Self {
        let mut edges = vec![vec![]; vertex_num];
        for (v0, v1) in edges_list {
            edges[v0].push(v1);
            edges[v1].push(v0);
        }
        Self { vertex_num, edges }
    }
    fn bipartite(&self) -> Option<Vec<bool>> {
        let mut x: Vec<Option<bool>> = vec![None; self.vertex_num];

        for i in 0..self.vertex_num {
            // ignore case
            if x[i].is_some() {
                continue;
            }
            // assign bool to all vertex in component which contains i
            let mut queue: VecDeque<usize> = VecDeque::new();
            x[i] = Some(true);
            queue.push_back(i);

            while let Some(next) = queue.pop_front() {
                let b0 = x[next].unwrap();
                for neighbor in &self.edges[next] {
                    if let Some(b1) = x[*neighbor] {
                        if b0 == b1 {
                            return None;
                        }
                    } else {
                        x[*neighbor] = Some(!b0);
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        // it shuould succeed
        let graph = x.into_iter().collect::<Option<Vec<bool>>>().unwrap();
        Some(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_bipartitegraph_test() {
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(1, vec![]).bipartite(),
            Some(vec![true])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(2, vec![]).bipartite(),
            Some(vec![true, true])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(2, vec![(0, 1)]).bipartite(),
            Some(vec![true, false])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(3, vec![(0, 1)]).bipartite(),
            Some(vec![true, false, true])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(3, vec![(0, 1), (1, 2)]).bipartite(),
            Some(vec![true, false, true])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(3, vec![(0, 1), (1, 2), (2, 0)]).bipartite(),
            None
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(4, vec![(0, 1), (1, 2), (2, 3)]).bipartite(),
            Some(vec![true, false, true, false])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(4, vec![(0, 1), (1, 3), (0, 2), (2, 3)])
                .bipartite(),
            Some(vec![true, false, false, true])
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(4, vec![(0, 1), (1, 3), (0, 2), (2, 3), (1, 2)])
                .bipartite(),
            None
        );
        assert_eq!(
            AdjacencyList::from_unordered_edgelist(4, vec![(0, 1), (1, 3), (0, 2), (2, 3), (0, 3)])
                .bipartite(),
            None
        );
    }
}

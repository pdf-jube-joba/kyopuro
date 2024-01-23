use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, a: usize, b: usize, c: usize,
        d: [[usize; n]; n],
    }

    println!("{}", min_cost(n, (a, b, c), d))
}

fn min_cost(n: usize, (a, b, c): (usize, usize, usize), d: Vec<Vec<usize>>) -> usize {
    let one_to_i: Vec<Vec<(usize, usize)>> = (0..n)
        .map(|i| (0..n).map(|j| (j, d[i][j] * a)).collect_vec())
        .collect_vec();

    let min_cost_one_to_i = dijkstra(n, 0, &one_to_i);

    let i_to_n: Vec<Vec<(usize, usize)>> = (0..n)
        .map(|i| (0..n).map(|j| (j, d[i][j] * b + c)).collect_vec())
        .collect_vec();

    let min_cost_i_to_n = dijkstra(n, n - 1, &i_to_n);

    (0..n)
        .map(|i| min_cost_one_to_i[i].unwrap() + min_cost_i_to_n[i].unwrap())
        .min()
        .unwrap()
}

// number... numbe of vertex
// edge_cost[i] = {(j, cost of path i to j)}
// return x[i] = min cost of 1 to i
fn dijkstra(
    number: usize,
    first: usize,
    edge_cost: &Vec<Vec<(usize, usize)>>,
) -> Vec<Option<usize>> {
    let mut costs: Vec<Option<usize>> = vec![None; number];
    costs[first] = Some(0);
    // priority queue of (cost of min path of 1 to i, i)
    let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    queue.push((Reverse(0), first));
    while let Some((Reverse(cost), next)) = queue.pop() {
        for &(neighbor, cost_to_neighbor) in &edge_cost[next] {
            if neighbor == next {
                continue;
            }
            let alt = cost_to_neighbor + cost;
            // if costs[neighbor] is none or (costs[neighbor] = Some(d) && d > alt)
            if costs[neighbor]
                .filter(|prev_cost| *prev_cost <= alt)
                .is_none()
            {
                costs[neighbor] = Some(alt);
                queue.push((Reverse(alt), neighbor));
            }
        }
    }
    costs
}

#[cfg(test)]
mod tests {
    use super::*;
    fn convert(n: usize, a: Vec<(usize, usize, usize)>) -> Vec<Vec<(usize, usize)>> {
        let mut graph = vec![vec![]; n];
        for (i, j, cost) in a {
            graph[i].push((j, cost));
        }
        graph
    }
    #[test]
    fn dijkstra_test() {
        //    6
        // 0 -- 2
        //1| ／4| 1
        // 1 -- 3
        //   1
        let num = 4;
        let graph = vec![(0, 1, 1), (0, 2, 6), (1, 2, 4), (1, 3, 1), (3, 2, 1)];
        let graph = convert(num, graph);

        let res = dijkstra(num, 0, &graph);
        assert_eq!(res, vec![Some(0), Some(1), Some(3), Some(2)])
    }
}

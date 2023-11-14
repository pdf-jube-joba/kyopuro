#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    from_v: usize,
    to_v: usize,
    distance: usize,
}

impl std::cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> std::option::Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl std::cmp::Ord for Edge {
    fn cmp(&self, other: &Edge) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn dijkstra(graph: Vec<Vec<Edge>>) -> Vec<usize> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let n = graph.len();
    debug_assert!(n > 0);
    let mut v: Vec<Option<usize>> = vec![None; n];
    v[0] = Some(0);
    let mut nexts: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();

    for edge in &graph[0] {
        nexts.push(Reverse(edge.clone()));
    }

    while let Some(Reverse(edge)) = nexts.pop() {
        let dist = edge.distance + v[edge.from_v].unwrap();
        match v[edge.to_v] {
            None => {
                v[edge.to_v] = Some(dist);
            }
            Some(dist2) if dist2 > dist => {
                v[edge.to_v] = Some(dist);
            }
            _ => {
                continue;
            }
        }
        for edge_to_v in &graph[edge.to_v] {
            nexts.push(Reverse(edge_to_v.clone()));
        }
    }

    v.into_iter().map(|v| v.unwrap()).collect()
}

fn main() {
    let list = input();
    let costs = dijkstra(list);
    for i in 0..costs.len() {
        println!("{} {}", i, costs[i]);
    }
}

fn input() -> Vec<Vec<Edge>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    let mut v = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let v = buf
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let ind = v[0];
            let deg = v[1];
            (
                ind,
                (0..deg).map(|i| Edge {
                    from_v: ind,
                    to_v: v[2 * i + 2],
                    distance: v[2 * i + 3],
                }).collect(),
            )
        })
        .collect::<Vec<(usize, Vec<Edge>)>>();
    v.sort_by(|(i1, _), (i2, _)| i1.cmp(&i2));
    v.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn from_vec(v: Vec<Vec<(usize, usize)>>) -> Vec<Vec<Edge>> {
        v.into_iter().enumerate().map(|(i, v)| {
            v.into_iter().map(|(j, d)| Edge {
                from_v: i,
                to_v: j,
                distance: d,
            }).collect()
        }).collect()
    }
    #[test]
    fn dijk_test() {
        let graph = vec![vec![]];
        assert_eq!(dijkstra(from_vec(graph)), vec![0]);

        let graph = vec![vec![(1, 0)], vec![]];
        assert_eq!(dijkstra(from_vec(graph)), vec![0, 0]);

        let graph = vec![vec![(1, 2)], vec![]];
        assert_eq!(dijkstra(from_vec(graph)), vec![0, 2]);

        let graph = vec![vec![(1, 1), (2, 5)], vec![(2, 2)], vec![]];
        assert_eq!(dijkstra(from_vec(graph)), vec![0, 1, 3]);

        let graph = vec![vec![(1, 1), (2, 5)], vec![(2, 2)], vec![(0, 1), (1, 1)]];
        assert_eq!(dijkstra(from_vec(graph)), vec![0, 1, 3]);

        let graph = vec![
            vec![(1, 1), (2, 5)],
            vec![(2, 2)],
            vec![(0, 1), (1, 1)],
        ];
        assert_eq!(dijkstra(from_vec(graph)), vec![0, 1, 3]);
    }
}

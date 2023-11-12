struct UnionTree {
    vec: Vec<Option<usize>>,
}

impl UnionTree {
    fn new(n: usize) -> Self {
        Self { vec: vec![None; n] }
    }
    fn flat(&mut self) {
        for i in 0..self.vec.len() {
            let mut this = i;
            let mut change = vec![];
            while let Some(parent) = self.vec[this] {
                change.push(this);
                this = parent;
            }
            for v in change {
                self.vec[v] = Some(this);
            }
        }
    }
    fn find_root(&self, x: usize) -> usize {
        let mut this = x;
        while let Some(parent) = self.vec[this] {
            this = parent
        }
        this
    }
    fn is_eq(&self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
    fn union(&mut self, x: usize, y: usize) {
        let rootx = self.find_root(x);
        let rooty = self.find_root(y);
        if rootx != rooty {
            self.vec[rootx] = Some(rooty);
        }
    }
}

fn cost(graph: Vec<Vec<Option<usize>>>) -> usize {
    let n = graph.len();
    let vertexes = {
        let mut v = vec![];
        for i in 0..n {
            for j in 0..n {
                if let Some(cost) = graph[i][j] {
                    v.push((i, j, cost));
                }
            }
        }
        v.sort_by(|(_,_, cost1), (_,_, cost2)| cost1.cmp(&cost2));
        v
    };
    let mut uni = UnionTree::new(n * n);
    let mut min_cost = 0;

    for (i, j, cost) in vertexes {
        if !uni.is_eq(i,j) {
            min_cost += cost;
            uni.union(i, j);
        }
    }
    min_cost
}

fn main() {
    let graph = input();
    let cost = cost(graph);
    println!("{}", cost);
}

fn input() -> Vec<Vec<Option<usize>>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..n).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|str|{
            match str.parse::<usize>() {
                Ok(u) => Some(u),
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost_test() {
        let graph = vec![];
        assert_eq!(cost(graph), 0);

        let graph = vec![vec![None]];
        assert_eq!(cost(graph), 0);

        let graph = vec![
            vec![None, Some(2)],
            vec![Some(2), None],
        ];
        assert_eq!(cost(graph), 2);

        let graph = vec![
            vec![None, Some(1), Some(1)],
            vec![Some(1), None, Some(1)],
            vec![Some(1), Some(1), None],
        ];
        assert_eq!(cost(graph), 2);

        let graph = vec![
            vec![None, Some(1), Some(1)],
            vec![Some(1), None, Some(1)],
            vec![Some(1), Some(1), None],
        ];
        assert_eq!(cost(graph), 2);

        let graph = vec![
            vec![None, Some(1), Some(2)],
            vec![Some(1), None, Some(3)],
            vec![Some(2), Some(3), None],
        ];
        assert_eq!(cost(graph), 3);

        let graph = vec![
            vec![None, Some(1), Some(2)],
            vec![Some(1), None, Some(3)],
            vec![Some(2), Some(3), None],
        ];
        assert_eq!(cost(graph), 3);

        let graph = vec![
            vec![None, Some(2), Some(1)],
            vec![Some(2), None, Some(3)],
            vec![Some(1), Some(3), None],
        ];
        assert_eq!(cost(graph), 3);
    }
}
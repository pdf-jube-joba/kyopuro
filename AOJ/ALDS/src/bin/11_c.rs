fn bfs_min_dist(adj: Vec<Vec<usize>>) -> Vec<Option<usize>> {
    let n = adj.len();
    if n == 0 {
        return vec![];
    }
    let mut m_d = vec![None; n];

    let mut queue = std::collections::VecDeque::<(usize, usize)>::new();
    queue.push_back((0, 0));
    while let Some((next, dist)) = queue.pop_front() {
        // eprintln!("n {} d {} nowmd {:?}", next, dist, m_d[next]);
        let ref_to: &mut Option<usize> = &mut m_d.get_mut(next).unwrap();
        if ref_to.is_none() {
            *ref_to = Some(dist);
            for around in &adj[next] {
                queue.push_back((*around, dist + 1));
            }
        }
    }
    m_d
}

fn main() {
    let adj = input();
    let min_d = bfs_min_dist(adj);

    for i in 0..min_d.len() {
        println!(
            "{} {}",
            i + 1,
            if let Some(i) = min_d[i] {
                i.to_string()
            } else {
                "-1".to_owned()
            }
        );
    }
}

fn input() -> Vec<Vec<usize>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let v: Vec<_> = buf
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            (&v[2..]).iter().map(|i| i - 1).collect()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bfs_test() {
        // empty case
        let adj = vec![];
        assert_eq!(bfs_min_dist(adj), vec![]);

        // 1 vertex case
        let adj = vec![vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0)]);

        // 0 -> 1
        let adj = vec![vec![1], vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), Some(1)]);

        // 0  1
        let adj = vec![vec![], vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), None]);

        // 0 -> 1 -> 2
        let adj = vec![vec![1], vec![2], vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), Some(1), Some(2)]);

        // 0  -> (1, 2)
        let adj = vec![vec![1, 2], vec![], vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), Some(1), Some(1)]);

        // 0 -> 1
        // |    ↓
        // ` -> 2
        let adj = vec![vec![1, 2], vec![2], vec![]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), Some(1), Some(1)]);

        // 0 -> 1
        // ↑    ↓
        // ` - 2
        let adj = vec![vec![1], vec![2], vec![0]];
        assert_eq!(bfs_min_dist(adj), vec![Some(0), Some(1), Some(2)]);
    }
}

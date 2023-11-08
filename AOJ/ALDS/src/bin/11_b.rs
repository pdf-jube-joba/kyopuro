fn dfs(adj: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    if n == 0 {
        return (vec![], vec![]);
    }
    let mut d: Vec<Option<usize>> = vec![None; n];
    let mut f: Vec<Option<usize>> = vec![None; n];

    let mut stack: Vec<usize> = vec![];
    let mut time = 0;

    loop {
        if let Some(next) = stack.pop() {
            time += 1;
            if d[next].is_some() {
                if f[next].is_some() {
                    time -= 1;
                } else {
                    f[next] = Some(time);
                }
            } else {
                d[next] = Some(time);
                stack.push(next);
                stack.extend(adj[next].iter().rev().filter(|i| d[**i].is_none()));
            }
        } else if let Some(min_next) = d.iter().position(|d| d.is_none()) {
            stack.push(min_next);
        } else {
            break;
        }
    }

    (
        d.into_iter().map(|t| t.unwrap()).collect(),
        f.into_iter().map(|t| t.unwrap()).collect(),
    )
}

fn main() {
    let adj = input();
    
    let (d, f) = dfs(adj);

    let n = d.len();

    for i in 0..n {
        println!("{} {} {}", i+1, d[i], f[i]);
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
    fn dfs_test() {
        // empty case
        let adj = vec![];
        assert_eq!(dfs(adj), (vec![], vec![]));

        // 1 vertex case
        let adj = vec![vec![]];
        assert_eq!(dfs(adj), (vec![1], vec![2]));

        // 2 certex case 0 -> 1
        let adj = vec![vec![1], vec![]];
        assert_eq!(dfs(adj), (vec![1, 2], vec![4, 3]));

        // 2 vertex case 0  1
        let adj = vec![vec![], vec![]];
        assert_eq!(dfs(adj), (vec![1, 3], vec![2, 4]));

        // 0 -> 1 -> 2
        let adj = vec![vec![1], vec![2], vec![]];
        assert_eq!(dfs(adj), (vec![1, 2, 3], vec![6, 5, 4]));

        // 0 -> (1, 2)
        let adj = vec![vec![1, 2], vec![], vec![]];
        assert_eq!(dfs(adj), (vec![1, 2, 4], vec![6, 3, 5]));

        // 0 -> 1 -> 2 -> 0
        let adj = vec![vec![1], vec![2], vec![0]];
        assert_eq!(dfs(adj), (vec![1, 2, 3], vec![6, 5, 4]));
    }
}

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }
    println!("{}", max_delte(n, abc))
}

fn max_delte(n: usize, abc: Vec<(usize, usize, usize)>) -> usize {
    let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; n];

    for &(a, b, c) in &abc {
        dist[a][b] = c;
        dist[b][a] = c;
    }

    for i in 0..n {
        dist[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = std::cmp::min(dist[i][j], dist[i][k].saturating_add(dist[k][j]))
            }
        }
    }

    abc.into_iter()
        .filter(|&(a, b, c)| (0..n).any(|i| (i != a && i != b) && dist[a][i] + dist[i][b] <= c))
        .count()
}

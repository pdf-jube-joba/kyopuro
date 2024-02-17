use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, a: usize, b: usize,
    }
    for v in tile(n, a, b) {
        println!(
            "{}",
            v.into_iter().map(|b| if b { '.' } else { '#' }).join("")
        )
    }
}

fn tile(n: usize, a: usize, b: usize) -> Vec<Vec<bool>> {
    let mut tiles = vec![vec![false; b * n]; a * n];
    for i in 0..n {
        for i1 in 0..a {
            for j in 0..n {
                for j1 in 0..b {
                    tiles[a * i + i1][b * j + j1] = (i + j) % 2 == 0;
                }
            }
        }
    }
    tiles
}

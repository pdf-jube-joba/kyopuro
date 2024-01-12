use itertools::{iproduct, Itertools};
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: [Chars; n],
    }
    let s = read_str(s);
    println!("{}", count_ls(s));
}

fn read_str(str: Vec<Vec<char>>) -> Vec<Vec<bool>> {
    str.into_iter()
        .map(|line| {
            line.into_iter()
                .filter_map(|char| match char {
                    'o' => Some(true),
                    'x' => Some(false),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec()
}

fn count_ls(grid: Vec<Vec<bool>>) -> usize {
    let n = grid.len();
    let grid_row_num: Vec<usize> = (0..n)
        .map(|i| grid[i].iter().filter(|b| **b).count())
        .collect_vec();
    let grid_col_num: Vec<usize> = (0..n)
        .map(|i| {
            grid.iter()
                .map(|grid_row| grid_row[i])
                .filter(|b| *b)
                .count()
        })
        .collect_vec();
    iproduct!(0..n, 0..n)
        .filter_map(|(i, j)| {
            if grid[i][j] {
                Some((grid_row_num[i] - 1) * (grid_col_num[j] - 1))
            } else {
                None
            }
        })
        .sum()
}

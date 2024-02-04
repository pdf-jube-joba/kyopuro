use itertools::{iproduct, Itertools};
use proconio::marker::Chars;

const L: usize = 4;

fn main() {
    proconio::input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    }
    let p1 = conv(p1);
    let p2 = conv(p2);
    let p3 = conv(p3);
    println!("{}", if fillable(p1, p2, p3) { "Yes" } else { "No" })
}

fn conv(p: Vec<Vec<char>>) -> Vec<Vec<bool>> {
    p.into_iter()
        .map(|v1| {
            v1.into_iter()
                .map(|char| match char {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn fillable(p1: Vec<Vec<bool>>, p2: Vec<Vec<bool>>, p3: Vec<Vec<bool>>) -> bool {
    rec(vec![vec![None; L]; L], &mut vec![p1, p2, p3], 0).is_some()
}

// if false then both grid and ps is not change
fn rec(
    grid: Vec<Vec<Option<usize>>>,
    ps: &mut Vec<Vec<Vec<bool>>>,
    n: usize,
) -> Option<Vec<Vec<usize>>> {
    let l = L as isize;
    if ps.is_empty() {
        return grid
            .into_iter()
            .map(|line| line.into_iter().collect::<Option<Vec<_>>>())
            .collect::<Option<Vec<Vec<_>>>>();
    }
    let mut p = ps.pop().unwrap();
    // rotate 3 times
    for _ in 0..4 {
        for (x, y) in iproduct!(-l..=l, -l..=l) {
            if let Some(grid_success) = put(&p, (x, y), &grid, n) {
                if let Some(fil) = rec(grid_success, ps, n + 1) {
                    return Some(fil);
                }
            }
        }
        rotate_right(&mut p);
    }
    ps.push(p);
    None
}

fn put(
    p: &Vec<Vec<bool>>,
    (x, y): (isize, isize),
    grid: &mut Vec<Vec<Option<usize>>>,
    n: usize,
) -> Option<Vec<Vec<Option<usize>>>> {
    let l = p.len();
    let mut g: Vec<(usize, usize)> = vec![];
    for i in 0..l {
        for j in 0..l {
            if p[i][j] {
                let i1 = {
                    let i1: isize = i as isize + x;
                    if 0 <= i1 && i1 < l as isize {
                        i1 as usize
                    } else {
                        return None;
                    }
                };
                let j1 = {
                    let j1 = j as isize + y;
                    if 0 <= j1 && j1 < l as isize {
                        j1 as usize
                    } else {
                        return None;
                    }
                };
                if grid[i1][j1].is_some() {
                    return None;
                }
                g.push((i1, j1));
            }
        }
    }
    let mut grid = grid.clone();
    for (i, j) in g {
        grid[i][j] = Some(n);
    }
    Some(grid)
}

// ab    ac    ca
// cd => bd => db
fn rotate_right<T: Copy>(grid: &mut Vec<Vec<T>>) {
    let l = grid.len();
    for i in 0..l {
        for j in i + 1..l {
            let tmp = grid[i][j];
            grid[i][j] = grid[j][i];
            grid[j][i] = tmp;
        }
    }
    for i in 0..l {
        for j in 0..(l / 2) {
            grid[i].swap(l - 1 - j, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rot() {
        let mut v = vec![vec![1, 2], vec![3, 4]];
        rotate_right(&mut v);
        assert_eq!(v, vec![vec![3, 1], vec![4, 2]])
    }
    #[test]
    fn puttest() {
        // let grid = vec![vec![false, false], vec![true, true]];
        // let mino = vec![vec![true, false], vec![false, false]];
        // assert_eq!(
        //     put(&mino, (0, 0), &grid),
        //     Some(vec![vec![true, false], vec![true, true]])
        // );
        // assert_eq!(
        //     put(&mino, (0, 1), &grid),
        //     Some(vec![vec![false, true], vec![true, true]])
        // );
        // assert_eq!(put(&mino, (-1, 0), &grid), None);
        // assert_eq!(put(&mino, (1, 0), &grid), None);
    }
}

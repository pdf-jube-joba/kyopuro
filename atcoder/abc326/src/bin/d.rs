use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    let grid = grid_get(r, c);
    if let Some(grid) = grid {
        println!("Yes");
        for row in grid {
            for aij in row {
                print!("{}", aij.unwrap_or('.'));
            }
            println!()
        }
    } else {
        println!("No");
    }
}

fn first(n: usize) -> Vec<Option<char>> {
    vec![vec![None; n - 3], vec![Some('A'), Some('B'), Some('C')]]
        .into_iter()
        .flatten()
        .collect_vec()
}

fn first_of_vec_is_c(a: &[Option<char>], c: char) -> bool {
    let first: Option<char> = a.iter().find_map(|b| *b);
    if let Some(first) = first {
        c == first
    } else {
        true
    }
}

fn is_ok(r: &[char], c: &[char], grid: &Vec<Vec<Option<char>>>) -> bool {
    let n = r.len();
    for i in 0..grid.len() {
        // check first of row == r[i]
        let enum_row = grid[i].to_vec();
        if !first_of_vec_is_c(&enum_row, r[i]) {
            return false;
        };
    }
    for j in 0..n {
        // check first of col == c[i]
        let enum_col = (0..grid.len()).map(|i| grid[i][j]).collect_vec();
        if !first_of_vec_is_c(&enum_col, c[j]) {
            return false;
        }
        // check either duplicative element is exists in colums or not
        let mut enum_col = enum_col.into_iter().flatten().collect_vec();
        enum_col.sort();
        let l0 = enum_col.len();
        enum_col.dedup();
        if enum_col.len() != l0 {
            return false;
        }
    }
    true
}

fn grid_get(r: Vec<char>, c: Vec<char>) -> Option<Vec<Vec<Option<char>>>> {
    let n = r.len();
    let mut grid: Vec<Vec<Option<char>>> = vec![];
    loop {
        if is_ok(&r, &c, &grid) {
            // check finished or not
            if grid.len() == n {
                // check col has some char
                let b = (0..n).all(|j| (0..n).filter_map(|i| grid[i][j]).count() == 3);
                if b {
                    return Some(grid);
                }
                // else, goto 'l
            } else {
                grid.push(first(n));
                continue;
            }
        }
        'l: loop {
            let mut p = grid.pop()?;
            if p.next_permutation() {
                grid.push(p);
                break 'l;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_test() {
        let r = vec!['A', 'B', 'C', 'A', 'B'];
        let c = vec!['A', 'B', 'C', 'A', 'B'];
        assert!(is_ok(&r, &c, &vec![]));
        assert!(is_ok(
            &r,
            &c,
            &vec![vec![Some('A'), None, None, None, None],]
        ));
        assert!(!is_ok(
            &r,
            &c,
            &vec![vec![Some('B'), None, None, None, None],]
        ));
        grid_get(r, c);
    }
}

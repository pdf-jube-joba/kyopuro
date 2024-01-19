use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let grid = read_str(s);
    println!("{}", connected_components(grid));
}

fn read_str(str: Vec<Vec<char>>) -> Vec<Vec<bool>> {
    str.into_iter()
        .map(|line| {
            line.into_iter()
                .map(|char| match char {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn adjacent((h, w): (usize, usize), (i, j): (usize, usize)) -> Vec<(usize, usize)> {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter_map(|(dx, dy)| {
            // 0 <= i + dx < h
            let i0 = i as isize + dx;
            let j0 = j as isize + dy;
            if 0 <= i0 && i0 < h as isize && 0 <= j0 && j0 < w as isize && (i as isize, j as isize) != (i0, j0) {
                Some((i0 as usize, j0 as usize))
            } else {
                None
            }
        })
        .collect_vec()
}

fn connected_components(grid: Vec<Vec<bool>>) -> usize {
    let (h, w) = (grid.len(), grid[0].len());

    let mut components = vec![vec![None; w]; h];
    let mut components_num = 0;

    for i in 0..h {
        for j in 0..w {
            if components[i][j].is_some() || !grid[i][j] {
                continue;
            }
            let mut stack = vec![(i, j)];

            while let Some(next) = stack.pop() {
                if !grid[next.0][next.1] || components[next.0][next.1].is_some() {
                    continue;
                }
                components[next.0][next.1] = Some(components_num);
                stack.extend(adjacent((h, w), next));
            }
            components_num += 1;
        }
    }
    components_num
}

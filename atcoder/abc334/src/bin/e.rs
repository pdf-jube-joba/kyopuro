use itertools::iproduct;
use proconio::marker::Chars;
use std::collections::VecDeque;

const MOD: usize = 998_244_353;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        chars: [Chars; h],
    }
    let grid: Vec<Vec<bool>> = chars
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unimplemented!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (sum, red) = count(grid);

    let ans = (sum * fast_pow(red, MOD - 2, MOD)) % MOD;

    println!("{}", ans);
}

// a^m mod p
fn fast_pow(mut a: usize, mut m: usize, p: usize) -> usize {
    let mut res: usize = 1;
    while m > 0 {
        if m % 2 == 1 {
            res = res.checked_mul(a).unwrap() % p;
        }
        a = a.checked_mul(a).unwrap() % p;
        m >>= 1;
    }
    res
}

fn count(grid: Vec<Vec<bool>>) -> (usize, usize) {
    let (h, w) = (grid.len(), grid[0].len());

    let adj = |(i, j): (usize, usize)| {
        vec![
            if i > 0 { Some((i - 1, j)) } else { None },
            if i < h - 1 { Some((i + 1, j)) } else { None },
            if j > 0 { Some((i, j - 1)) } else { None },
            if j < w - 1 { Some((i, j + 1)) } else { None },
        ]
        .into_iter()
        .flatten()
    };

    let (grid_components, components_num, red_num): (Vec<Vec<Option<usize>>>, usize, usize) =
        {
            let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
            let mut comp_num = 0;
            let mut red_num = 0;

            for (i, j) in iproduct!(0..h, 0..w) {
                if grid_components[i][j].is_some() {
                    continue;
                }

                if !grid[i][j] {
                    red_num += 1;
                    continue;
                }

                let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

                while let Some(pt) = queue.pop_front() {
                    if !grid[pt.0][pt.1] || grid_components[pt.0][pt.1].is_some() {
                        continue;
                    }
                    grid_components[pt.0][pt.1] = Some(comp_num);
                    queue.extend(adj(pt)) //.filter(|pt2| {
                        // grid_components[pt2.0][pt2.1].is_none() && grid[pt2.0][pt2.1]
                    //}));
                }

                comp_num += 1;
            }

            (grid_components, comp_num, red_num)
        };

    let all_sum: usize = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| !grid[i][j])
        .map(|(i, j)| {
            let mut adj_comp: Vec<usize> = adj((i, j))
                .filter_map(|(i, j)| grid_components[i][j])
                .collect();
            adj_comp.sort();
            adj_comp.dedup();
            components_num + 1 - adj_comp.len()
        })
        .sum();

    (all_sum, red_num)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use proconio::source::once::OnceSource;

    use super::*;
    #[test]
    fn fast_pow_test() {
        assert_eq!(fast_pow(1, 0, 7), 1);
        assert_eq!(fast_pow(2, 0, 7), 1);
        assert_eq!(fast_pow(5, 0, 7), 1);
        assert_eq!(fast_pow(1, 1, 7), 1);
        assert_eq!(fast_pow(1, 5, 7), 1);
        assert_eq!(fast_pow(2, 1, 7), 2);
        assert_eq!(fast_pow(2, 2, 7), 4);
        assert_eq!(fast_pow(2, 3, 7), 1);
        assert_eq!(fast_pow(2, 4, 7), 2);
        assert_eq!(fast_pow(3, 1, 7), 3);
        assert_eq!(fast_pow(3, 2, 7), 2);
        assert_eq!(fast_pow(3, 3, 7), 6);
        assert_eq!(fast_pow(3, 4, 7), 4);

        for i in 1..10_000 {
            assert_eq!(i * fast_pow(i, MOD - 2, MOD) % MOD, 1);
        }
    }
    fn read_chars(str: &str) -> Vec<Vec<bool>> {
        let a = OnceSource::from(str);
        proconio::input! {
            from a,
            h: usize, w: usize,
            chars: [Chars; h],
        }
        chars
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unimplemented!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
    #[test]
    fn trivial_case() {
        for (i, j) in iproduct!(1..=4, 1..=4) {
            let mut grid = format!("{i} {j}");
            for _ in 0..i {
                grid.push('\n');
                grid.extend((0..j).map(|_| '.'));
            }
            let grid = read_chars(&grid);
            assert_eq!(count(grid), (i * j, i * j));
        }

        let mut grid = String::from("1000 1000");
        for _ in 0..1_000 {
            grid.push('\n');
            grid.extend((0..1_000).map(|_| '.'));
        }
        let grid = read_chars(&grid);
        assert_eq!(count(grid), (1_000 * 1_000, 1_000 * 1_000))
    }
    #[test]
    fn case_2_2() {
        let grid = "2 2\n#.\n..";
        assert_eq!(count(read_chars(grid)), (4, 3));
        let grid = "2 2\n.#\n..";
        assert_eq!(count(read_chars(grid)), (4, 3));
        let grid = "2 2\n..\n#.";
        assert_eq!(count(read_chars(grid)), (4, 3));
        let grid = "2 2\n..\n.#";
        assert_eq!(count(read_chars(grid)), (4, 3));

        let grid = "2 2\n##\n..";
        assert_eq!(count(read_chars(grid)), (2, 2));
        let grid = "2 2\n..\n##";
        assert_eq!(count(read_chars(grid)), (2, 2));
        let grid = "2 2\n#.\n#.";
        assert_eq!(count(read_chars(grid)), (2, 2));
        let grid = "2 2\n.#\n.#";
        assert_eq!(count(read_chars(grid)), (2, 2));
        let grid = "2 2\n#.\n.#";
        assert_eq!(count(read_chars(grid)), (2, 2));
        let grid = "2 2\n.#\n#.";
        assert_eq!(count(read_chars(grid)), (2, 2));
    }
    #[test]
    fn case_3_3() {
        let grid = "3 3\n#..\n...\n...";
        assert_eq!(count(read_chars(grid)), (14, 8));
        let grid = "3 3\n...\n.#.\n...";
        assert_eq!(count(read_chars(grid)), (12, 8));

        let grid = "3 3\n#..\n#..\n#..";
        assert_eq!(count(read_chars(grid)), (9, 6));
        let grid = "3 3\n#.#\n#.#\n#.#";
        assert_eq!(count(read_chars(grid)), (3, 3));

        let grid = "3 3\n#..\n.#.\n...";
        assert_eq!(count(read_chars(grid)), (15, 7));
        let grid = "3 3\n#.#\n.#.\n#.#";
        assert_eq!(count(read_chars(grid)), (12, 4));
        let grid = "3 3\n.##\n#.#\n##.";
        assert_eq!(count(read_chars(grid)), (3, 3));
    }
    #[test]
    fn random_tle() {
        let grid: Vec<Vec<bool>> = (0..1_000)
            .map(|_| {
                (0..1_000)
                    .map(|_| rand::random::<bool>())
                    .collect::<Vec<_>>()
            })
            .collect();
        count(grid);
    }
}

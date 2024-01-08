use std::collections::VecDeque;

use itertools::{iproduct, Itertools};

fn main() {}

fn random_grid(size: usize) -> Vec<Vec<bool>> {
    (0..size)
        .map(|_| (0..size).map(|_| rand::random()).collect())
        .collect()
}

fn adj((h, w): (usize, usize), (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    vec![
        if i > 0 { Some((i - 1, j)) } else { None },
        if j > 0 { Some((i, j - 1)) } else { None },
        if j < w - 1 { Some((i, j + 1)) } else { None },
        if i < h - 1 { Some((i + 1, j)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn compute_1(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_front() {
            if !grid[pt.0][pt.1] || grid_components[pt.0][pt.1].is_some() {
                continue;
            }
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(adj((h, w), pt));
        }

        comp_num += 1;
    }

    grid_components
}

fn compute_2(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_front() {
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(
                adj((h, w), pt)
                    .filter(|pt2| grid[pt2.0][pt2.1] && grid_components[pt2.0][pt2.1].is_none()),
            );
        }

        comp_num += 1;
    }

    grid_components
}

fn compute_3(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_front() {
            grid_components[pt.0][pt.1] = Some(comp_num);
            let res = adj((h, w), pt)
                .filter(|pt2| {
                    grid[pt2.0][pt2.1]
                        && grid_components[pt2.0][pt2.1].is_none()
                        && !queue.contains(pt2)
                })
                .collect::<Vec<_>>();
            queue.extend(res);
        }

        comp_num += 1;
    }

    grid_components
}

#[cfg(test)]
mod tests {
    use itertools::concat;

    use super::*;
    #[test]
    fn test() {
        let size = 10;
        let limit_time = std::time::Duration::from_micros(1000);
        let mut grids1 = vec![];
        let mut sum_time1 = std::time::Duration::ZERO;
        let mut max_time1 = std::time::Duration::ZERO;
        let mut grids2 = vec![];
        let mut sum_time2 = std::time::Duration::ZERO;
        let mut max_time2 = std::time::Duration::ZERO;
        let mut grids3 = vec![];
        let mut sum_time3 = std::time::Duration::ZERO;
        let mut max_time3 = std::time::Duration::ZERO;
        for _ in 0..10_000 {
            let grid = random_grid(size);

            let start_time1 = std::time::Instant::now();
            let res1 = compute_1(grid.clone());
            let time1 = std::time::Instant::now() - start_time1;

            let start_time2 = std::time::Instant::now();
            let res2 = compute_2(grid.clone());
            let time2 = std::time::Instant::now() - start_time2;

            let start_time3 = std::time::Instant::now();
            let res3 = compute_3(grid.clone());
            let time3 = std::time::Instant::now() - start_time3;

            if time1 > limit_time {
                grids1.push((time1, grid.clone()));
            }
            if time2 > limit_time {
                grids2.push((time2, grid.clone()));
            }
            if time3 > limit_time {
                grids3.push((time3, grid.clone()));
            }
            sum_time1 += time1;
            max_time1 = std::cmp::max(max_time1, time1);
            sum_time2 += time2;
            max_time2 = std::cmp::max(max_time2, time2);
            sum_time3 += time3;
            max_time3 = std::cmp::max(max_time3, time3);

            assert_eq!(res1, res2);
        }

        println!("sum: {:?} {:?} {:?}", sum_time1, sum_time2, sum_time3);
        println!("max: {:?} {:?} {:?}", max_time1, max_time2, max_time3);

        println!("1");
        for (time, grid) in grids1 {
            println!("{:?}", time);
            for i in 0..size {
                for j in 0..size {
                    print!("{}", if grid[i][j] { '#' } else { '.' })
                }
                println!()
            }
        }

        println!("2");
        for (time, grid) in grids2 {
            println!("{:?}", time);
            for i in 0..size {
                for j in 0..size {
                    print!("{}", if grid[i][j] { '#' } else { '.' })
                }
                println!()
            }
        }

        println!("3");
        for (time, grid) in grids3 {
            println!("{:?}", time);
            for i in 0..size {
                for j in 0..size {
                    print!("{}", if grid[i][j] { '#' } else { '.' })
                }
                println!()
            }
        }
    }
    fn read_str(s: &str) -> Vec<Vec<bool>> {
        s.lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|char| match char {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    }
    #[test]
    fn all_true_test() {
        let grid = vec![vec![true; 15]; 15];
        let start_time1 = std::time::Instant::now();
        compute_1(grid.clone());
        let time1 = std::time::Instant::now() - start_time1;
        let start_time2 = std::time::Instant::now();
        compute_2(grid);
        let time2 = std::time::Instant::now() - start_time2;
        // 130μs vs 56s 1000*1000倍違う！
        println!("{:?} {:?}", time1, time2);
    }
    #[test]
    fn all_true_view_queue_stack() {
        let size = 4;
        // 1 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        let start_time = std::time::Instant::now();
        while let Some(pt) = queue.pop_front() {
            if checked[pt.0][pt.1] {
                continue;
            }
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt));
        }

        let time1 = std::time::Instant::now() - start_time;

        // 2 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        let start_time = std::time::Instant::now();
        while let Some(pt) = queue.pop_front() {
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt).filter(|pt2| !checked[pt2.0][pt2.1]));
        }

        let time2 = std::time::Instant::now() - start_time;

        // 3 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        let start_time = std::time::Instant::now();
        while let Some(pt) = queue.pop_back() {
            if checked[pt.0][pt.1] {
                continue;
            }
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt));
        }

        let time3 = std::time::Instant::now() - start_time;

        // 4 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        let start_time = std::time::Instant::now();
        while let Some(pt) = queue.pop_back() {
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt).filter(|pt2| !checked[pt2.0][pt2.1]));
        }

        let time4 = std::time::Instant::now() - start_time;

        println!("{:?} {:?} {:?} {:?}", time1, time2, time3, time4);
    }
    #[test]
    fn print_44_all_true() {
        let size = 4;

        let convert = |(i, j): (usize, usize)| -> char {
            let str = format!("{:x}", (4 * i + j));
            str.chars().next().unwrap()
        };

        let print_vec = |a: &[(usize, usize)]| -> String {
            format!("[{}]", a.iter().map(|i| convert(*i).to_string()).join(", "))
        };
        assert_eq!(print_vec(&[(0, 1), (2, 2), (1, 1)]), "[1, a, 5]");

        let concat =
            |head: (usize, usize), queue: &VecDeque<(usize, usize)>| -> Vec<(usize, usize)> {
                let mut v = vec![head];
                v.extend(queue.iter());
                v
            };

        convert((0, 0));
        // 1 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        println!("1");
        while let Some(pt) = queue.pop_front() {
            // println!(
            //     "| {} | {} | {} |",
            //     convert(pt),
            //     print_vec(&concat(pt, &queue)),
            //     {
            //         if checked[pt.0][pt.1] {
            //             "".to_string()
            //         } else {
            //             print_vec(&adj((size, size), pt).collect::<Vec<_>>())
            //         }
            //     },
            // );
            if checked[pt.0][pt.1] {
                continue;
            }
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt));
        }

        // 2 case
        let mut checked = vec![vec![false; size]; size];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));

        println!("2");
        while let Some(pt) = queue.pop_front() {
            println!(
                "| {} | {} | {} | {} |",
                convert(pt),
                print_vec(&concat(pt, &queue)),
                print_vec(
                    &adj((size, size), pt)
                        .filter(|pt2| !checked[pt2.0][pt2.1])
                        .collect::<Vec<_>>()
                ),
                print_vec(&adj((size, size), pt).collect::<Vec<_>>()),
            );
            checked[pt.0][pt.1] = true;
            queue.extend(adj((size, size), pt).filter(|pt2| !checked[pt2.0][pt2.1]));
        }
    }
}

fn main() {
    let input = input();
    println!("{}", compute_part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    U,
    R,
    D,
    L,
}

fn compute_part1(input: Vec<(Direction, usize, String)>) -> usize {
    let input: Vec<(Direction, usize)> = input.iter().map(|(d, l, _)| (d.clone(), *l)).collect();
    let mut trench = get_trench_loop(&input);
    // for i in 0..trench.len() {
    //     for j in 0..trench[0].len() {
    //         print!("{}", if trench[i][j] {'#'} else {'.'})
    //     }
    //     println!()
    // }
    fill(&mut trench);
    // println!();
    // for i in 0..trench.len() {
    //     for j in 0..trench[0].len() {
    //         print!("{}", if trench[i][j] {'#'} else {'.'})
    //     }
    //     println!()
    // }
    trench.into_iter().flatten().filter(|b| *b).count()
}

fn input() -> Vec<(Direction, usize, String)> {
    let string = std::fs::read_to_string("inout/day18.in").unwrap();
    string
        .lines()
        .map(|line| {
            let v: Vec<_> = line.split_whitespace().collect();
            (
                match v[0] {
                    "U" => Direction::U,
                    "R" => Direction::R,
                    "D" => Direction::D,
                    "L" => Direction::L,
                    _ => unreachable!("{}", &v[0]),
                },
                v[1].parse::<usize>().unwrap(),
                v[2].to_owned(),
            )
        })
        .collect::<Vec<_>>()
}

fn get_trench_loop(input: &Vec<(Direction, usize)>) -> Vec<Vec<bool>> {
    let mut now: (isize, isize) = (0, 0);
    let mut min_h = 0;
    let mut max_h = 0;
    let mut min_w = 0;
    let mut max_w = 0;
    for (d, l) in input {
        match d {
            Direction::U => {
                now.0 -= *l as isize;
            }
            Direction::D => {
                now.0 += *l as isize;
            }
            Direction::L => {
                now.1 -= *l as isize;
            }
            Direction::R => {
                now.1 += *l as isize;
            }
        }
        min_h = std::cmp::min(min_h, now.0);
        max_h = std::cmp::max(max_h, now.0);
        min_w = std::cmp::min(min_w, now.1);
        max_w = std::cmp::max(max_w, now.1);
    }
    let h = (max_h - min_h) as usize + 1;
    let w = (max_w - min_w) as usize + 1;

    println!("{min_h} {max_h} {min_w} {max_w} {h} {w}");

    let mut trench = vec![vec![false; w]; h];
    let mut now: (usize, usize) = ((-min_h) as usize, (-min_w) as usize);
    trench[now.0][now.1] = true;
    for (d, l) in input {
        match d {
            Direction::U => {
                for i in 0..*l {
                    trench[now.0 - i][now.1] = true;
                }
                now.0 -= *l;
            }
            Direction::D => {
                for i in 0..*l {
                    trench[now.0 + i][now.1] = true;
                }
                now.0 += *l;
            }
            Direction::L => {
                for i in 0..*l {
                    trench[now.0][now.1 - i] = true;
                }
                now.1 -= *l;
            }
            Direction::R => {
                for i in 0..*l {
                    trench[now.0][now.1 + i] = true;
                }
                now.1 += *l;
            }
        }
        trench[now.0][now.1] = true;
    }
    trench
}

fn fill(trench: &mut Vec<Vec<bool>>) {
    let h = trench.len();
    let w = trench[0].len();
    let mut nextpts: Vec<(usize, usize)> = vec![(1, (0..w).find(|&j| trench[1][j]).unwrap() + 1)];
    while let Some(nextpt) = nextpts.pop() {
        if !trench[nextpt.0][nextpt.1] {
            trench[nextpt.0][nextpt.1] = true;
            if 0 < nextpt.0 {
                nextpts.push((nextpt.0 - 1, nextpt.1));
            }
            if nextpt.0 < h - 1 {
                nextpts.push((nextpt.0 + 1, nextpt.1));
            }
            if 0 < nextpt.1 {
                nextpts.push((nextpt.0, nextpt.1 - 1));
            }
            if nextpt.1 < w - 1 {
                nextpts.push((nextpt.0, nextpt.1 + 1));
            }
        }
    }
}

fn compute_part2(input: Vec<(Direction, usize, String)>) -> usize {
    let input: Vec<_> = input
        .into_iter()
        .map(|(d, _, string)| {
            let dir = match &string[7..8] {
                "0" => Direction::R,
                "1" => Direction::D,
                "2" => Direction::L,
                "3" => Direction::U,
                _ => unreachable!("{}", &string[7..8]),
            };
            let len = usize::from_str_radix(&string[2..7], 16).unwrap();
            (dir, len)
        })
        .collect();
    todo!()
}

fn compressed_coordinate(v: &Vec<(Direction, usize)>) -> (Vec<usize>, Vec<usize>, Vec<Vec<bool>>) {
    let mut now: (isize, isize) = (0, 0);
    let mut min_h = 0;
    let mut max_h = 0;
    let mut min_w = 0;
    let mut max_w = 0;
    let mut exist_h = vec![0];
    let mut exist_w = vec![0];
    for (d, l) in v {
        match d {
            Direction::U => {
                now.0 -= *l as isize;
                exist_h.push(now.0);
            }
            Direction::D => {
                now.0 += *l as isize;
                exist_h.push(now.0);
            }
            Direction::L => {
                now.1 -= *l as isize;
                exist_w.push(now.1);
            }
            Direction::R => {
                now.1 += *l as isize;
                exist_w.push(now.1);
            }
        }
        min_h = std::cmp::min(min_h, now.0);
        max_h = std::cmp::max(max_h, now.0);
        min_w = std::cmp::min(min_w, now.1);
        max_w = std::cmp::max(max_w, now.1);
    }
    let h = (max_h - min_h) as usize + 1;
    let w = (max_w - min_w) as usize + 1;

    println!("{min_h} {max_h} {min_w} {max_w} {h} {w}");

    let mut exist_h: Vec<_> = exist_h.into_iter().map(|h| (h + (-min_h)) as usize).collect();
    exist_h.sort();

    let mut exist_w: Vec<_> = exist_w.into_iter().map(|w| (w + (-min_w)) as usize).collect();
    exist_w.sort();

    let mut now: (usize, usize) = ((-min_h) as usize, (-min_w) as usize);
    // for (d, l) in input {
    //     match d {
    //         Direction::U => {
    //             for i in 0..*l {
    //                 trench[now.0 - i][now.1] = true;
    //             }
    //             now.0 -= *l;
    //         }
    //         Direction::D => {
    //             for i in 0..*l {
    //                 trench[now.0 + i][now.1] = true;
    //             }
    //             now.0 += *l;
    //         }
    //         Direction::L => {
    //             for i in 0..*l {
    //                 trench[now.0][now.1 - i] = true;
    //             }
    //             now.1 -= *l;
    //         }
    //         Direction::R => {
    //             for i in 0..*l {
    //                 trench[now.0][now.1 + i] = true;
    //             }
    //             now.1 += *l;
    //         }
    //     }
    //     trench[now.0][now.1] = true;
    // }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_part1_test() {
        let order: Vec<(Direction, usize, String)> = vec![
            (Direction::R, 6, "".to_string()),
            (Direction::D, 5, "".to_string()),
            (Direction::L, 2, "".to_string()),
            (Direction::D, 2, "".to_string()),
            (Direction::R, 2, "".to_string()),
            (Direction::D, 2, "".to_string()),
            (Direction::L, 5, "".to_string()),
            (Direction::U, 2, "".to_string()),
            (Direction::L, 1, "".to_string()),
            (Direction::U, 2, "".to_string()),
            (Direction::R, 2, "".to_string()),
            (Direction::U, 3, "".to_string()),
            (Direction::L, 2, "".to_string()),
            (Direction::U, 2, "".to_string()),
        ];
        let c = compute_part1(order);
        assert_eq!(c, 62);
    }
}

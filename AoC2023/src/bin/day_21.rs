use std::collections::{HashMap, HashSet};

fn main() {
    let (start, map) = input();
    println!("{}", compute_part2(start, map));
}

fn compute_part1(start: (usize, usize), map: Vec<Vec<bool>>) -> usize {
    let h = map.len();
    let w = map[0].len();
    let mut step_pre: Vec<Vec<bool>> = vec![vec![false; w]; h];
    step_pre[start.0][start.1] = true;
    for _ in 0..64 {
        step_pre = step_one(&map, &step_pre);
    }
    step_pre.into_iter().flatten().filter(|b| *b).count()
}

fn step_one(map: &Vec<Vec<bool>>, step: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let h = map.len();
    let w = map[0].len();
    let mut now = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if !map[i][j]
                && ((0 < i && step[i - 1][j])
                    || (i + 1 < h && step[i + 1][j])
                    || (0 < j && step[i][j - 1])
                    || (j + 1 < w && step[i][j + 1]))
            {
                now[i][j] = true;
            }
        }
    }
    now
}

fn compute_board(start: (usize, usize), map: &Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let h = map.len();
    let w = map[0].len();
    let mut step: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    step[start.0][start.1] = Some(0);
    let mut count = 1;
    let max_of_board = max_of_board(map);
    while count < max_of_board {
        let mut next_step = vec![vec![None; w]; h];
        for i in 0..h {
            for j in 0..w {
                if !map[i][j] {
                    if let Some(step_num) = step[i][j] {
                        next_step[i][j] = Some(step_num);
                    } else {
                        if 0 < i {
                            if let Some(step_num) = step[i - 1][j] {
                                next_step[i][j] = Some(step_num + 1);
                                count += 1;
                                continue;
                            }
                        }
                        if i + 1 < h {
                            if let Some(step_num) = step[i + 1][j] {
                                next_step[i][j] = Some(step_num + 1);
                                count += 1;
                                continue;
                            }
                        }
                        if 0 < j {
                            if let Some(step_num) = step[i][j - 1] {
                                next_step[i][j] = Some(step_num + 1);
                                count += 1;
                                continue;
                            }
                        }
                        if j + 1 < w {
                            if let Some(step_num) = step[i][j + 1] {
                                next_step[i][j] = Some(step_num + 1);
                                count += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        step = next_step;
    }
    step
}

fn print(a: &Vec<Vec<bool>>) {
    let h = a.len();
    let w = a[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{}", if a[i][j] { 'O' } else { '.' })
        }
        println!()
    }
    println!()
}

fn max_of_board(map: &Vec<Vec<bool>>) -> usize {
    let h = map.len();
    let w = map[0].len();
    let mut can_be_reach = vec![vec![false; w]; h];
    can_be_reach[0][0] = true;
    loop {
        let mut b = true;
        for i in 0..h {
            for j in 0..w {
                if !map[i][j]
                    && ((0 < i && can_be_reach[i - 1][j])
                        || (i + 1 < h && can_be_reach[i + 1][j])
                        || (0 < j && can_be_reach[i][j - 1])
                        || (j + 1 < w && can_be_reach[i][j + 1]))
                    && !can_be_reach[i][j]
                {
                    can_be_reach[i][j] = true;
                    b = false;
                }
            }
        }
        if b {
            break;
        }
    }
    can_be_reach.into_iter().flatten().filter(|b| *b).count()
}

const STEP_NUM: usize = 26501365;

// assume that
// - top row, bottom row, left col and right col are empty
// - row of S and col of S  are empty
// - for any start from the top/bottom row or left/right col, all reachable squares are reached after h+w steps
fn compute_part2(start: (usize, usize), map: Vec<Vec<bool>>) -> usize {
    step_many(start, &map, STEP_NUM)
}

fn dist_tile(
    (h, w): (usize, usize),
    ((i1, j1), (x1, y1)): ((isize, isize), (usize, usize)),
    ((i2, j2), (x2, y2)): ((isize, isize), (usize, usize)),
) -> usize {
    let pt1: (isize, isize) = (i1 * h as isize + x1 as isize, j1 * w as isize + y1 as isize);
    let pt2: (isize, isize) = (i2 * h as isize + x2 as isize, j2 * w as isize + y2 as isize);
    pt1.0.abs_diff(pt2.0) + pt1.1.abs_diff(pt2.1)
}

struct Memoize {
    start: (usize, usize),
    map: Vec<Vec<bool>>,
    board_closest: HashMap<(usize, usize), Vec<Vec<Option<usize>>>>,
    board_closest_step: HashMap<((usize, usize), usize), usize>,
    board_max_even: usize,
    board_max_odd: usize,
}

impl Memoize {
    fn new(start: (usize, usize), map: &Vec<Vec<bool>>) -> Self {
        let (h, w) = (map.len(), map[0].len());
        let mut board_closest = HashMap::new();
        for si in vec![0, start.0, h - 1] {
            for sj in vec![0, start.1, w - 1] {
                board_closest.insert((si, sj), compute_board((si, sj), map));
            }
        }
        let board_closest_step = HashMap::new();

        let start_board = board_closest.get(&start).unwrap().clone();

        Self {
            start,
            map: map.clone(),
            board_closest,
            board_closest_step,
            board_max_even: start_board
                .iter()
                .flatten()
                .filter(|b| matches!(b, Some(b) if b % 2 == 0))
                .count(),
            board_max_odd: start_board
                .iter()
                .flatten()
                .filter(|b| matches!(b, Some(b) if b % 2 != 0))
                .count(),
        }
    }
    fn tile_compute(&mut self, (i, j): (isize, isize), step_num: usize) -> usize {
        use std::cmp::Ordering;
        let (h, w) = (self.map.len(), self.map[0].len());
        let closest_point_in_tile = {
            let si = match i.cmp(&0) {
                Ordering::Equal => self.start.0,
                Ordering::Less => h - 1,
                Ordering::Greater => 0,
            };
            let sj = match j.cmp(&0) {
                Ordering::Equal => self.start.1,
                Ordering::Less => w - 1,
                Ordering::Greater => 0,
            };
            (si, sj)
        };
        let dist = dist_tile(
            (h, w),
            ((0, 0), self.start),
            ((i, j), closest_point_in_tile),
        );

        // too far
        if step_num < dist {
            return 0;
        }

        let remain_step = step_num - dist;
        // inner case
        // overestimate distance
        if remain_step > h + w {
            let remain_step_from_start =
                step_num - dist_tile((h, w), ((0, 0), self.start), ((i, j), self.start));
            if remain_step_from_start % 2 == 0 {
                return self.board_max_even;
            } else {
                return self.board_max_odd;
            }
        }

        if let Some(num) = self.board_closest_step.get(&(closest_point_in_tile, remain_step)) {
            return *num;
        }

        // outer case
        let board = self.board_closest.get(&closest_point_in_tile).unwrap();
        let count = board
            .iter()
            .flatten()
            .filter(|b| matches!(b, Some(b) if *b <= remain_step && *b % 2 == remain_step % 2))
            .count();
        println!("{i} {j} {count}");
        self.board_closest_step.insert((closest_point_in_tile, remain_step), count);
        count
    }
}

fn step_many(start: (usize, usize), map: &Vec<Vec<bool>>, step_num: usize) -> usize {
    let (h, w) = (map.len(), map[0].len());
    let mut memo = Memoize::new(start, map);
    // enumerate reacheable tile (i,j) ... over estimate (|i| - 1) * h + (|j| - 1) * w <= step_num
    let mut count = 0;
    let max_i = (step_num / h + 1);
    for i in -(max_i as isize)..=max_i as isize {
        let max_j = (step_num - (i.unsigned_abs() - 1) * h) / w;
        for j in -(max_j as isize)..=max_j as isize {
            let count_tile = memo.tile_compute((i, j), step_num);
            count += count_tile;
        }
    }
    count
}

fn input() -> ((usize, usize), Vec<Vec<bool>>) {
    let string = std::fs::read_to_string("inout/day21.in").unwrap();
    let mut start = (0, 0);
    let map = string
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| match char {
                    '#' => true,
                    '.' => false,
                    'S' => {
                        start.0 = i;
                        start.1 = j;
                        false
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (start, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn read(string: String) -> ((usize, usize), Vec<Vec<bool>>) {
        let mut start = (0, 0);
        let map = string
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(j, char)| match char {
                        '#' => true,
                        '.' => false,
                        'S' => {
                            start.0 = i;
                            start.1 = j;
                            false
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (start, map)
    }
    #[test]
    fn step_test() {
        let input = "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";
        let (start, map) = read(input.to_owned());
        let h = map.len();
        let w = map[0].len();
        let mut step_pre: Vec<Vec<bool>> = vec![vec![false; w]; h];
        step_pre[start.0][start.1] = true;

        step_pre = step_one(&map, &step_pre);
        assert_eq!(step_pre.iter().flatten().filter(|b| **b).count(), 2);

        step_pre = step_one(&map, &step_pre);
        assert_eq!(step_pre.iter().flatten().filter(|b| **b).count(), 4);

        step_pre = step_one(&map, &step_pre);
        assert_eq!(step_pre.iter().flatten().filter(|b| **b).count(), 6);
    }
    #[test]
    fn board_compute_test() {
        let input = "...........
                        .....###.#.
                        .###.##..#.
                        ..#.#...#..
                        ....#.#....
                        .##..S####.
                        .##..#...#.
                        .......##..
                        .##.#.####.
                        .##..##.##.
                        ...........";
        let (start, map) = read(input.to_owned());
        assert_eq!(
            max_of_board(&map),
            map.iter().flatten().filter(|b| !**b).count()
        );
        assert_eq!(
            compute_board(start, &map)
                .into_iter()
                .flatten()
                .filter(|b| b.is_some())
                .count(),
            max_of_board(&map)
        );
    }
    #[test]
    fn board_count_test() {
        let input = "\
        .....
        .#.#.
        ..S..
        .#.#.
        .....";
        let (start, map) = read(input.to_owned());
        let mut memo = Memoize::new(start, &map);
        assert_eq!(memo.tile_compute((0, 0), 0), 1);
        assert_eq!(memo.tile_compute((0, 0), 2), 5);
        assert_eq!(memo.tile_compute((0, 0), 5), 12);
        assert_eq!(memo.tile_compute((0, 0), 100), 9);
        assert_eq!(memo.tile_compute((-1, 0), 3), 1);
        assert_eq!(memo.tile_compute((-1, 0), 7), 7);
    }
}

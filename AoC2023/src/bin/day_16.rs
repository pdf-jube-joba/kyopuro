fn main() {
    let input = input();
    println!("{}", compute_part2(input));
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Space,
    SplitVertical,
    SplitHorizontal,
    MirrorLTtoRD,
    MirrorLDtoRT,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn compute_part1(input: Vec<Vec<Tile>>) -> usize {
    start_count(&input, ((0, 0), Direction::Right))
}

fn start_rec(
    input: &Vec<Vec<Tile>>,
    visited: &mut Vec<Vec<bool>>,
    start: ((usize, usize), Direction),
    checked_start: &mut Vec<((usize, usize), Direction)>,
) {
    let h = input.len();
    let w = input[0].len();
    checked_start.push(start.clone());
    let mut now = start;
    loop {
        visited[now.0 .0][now.0 .1] = true;
        match &input[now.0 .0][now.0 .1] {
            Tile::Space => {}
            Tile::SplitVertical if now.1 == Direction::Right || now.1 == Direction::Left => {
                if now.0 .0 > 0 {
                    let next_start = ((now.0 .0 - 1, now.0 .1), Direction::Up);
                    if !checked_start.contains(&next_start) {
                        start_rec(input, visited, next_start, checked_start);
                    }
                }
                if now.0 .0 < h - 1 {
                    let next_start = ((now.0 .0 + 1, now.0 .1), Direction::Down);
                    if !checked_start.contains(&next_start) {
                        start_rec(input, visited, next_start, checked_start);
                    }
                }
                break;
            }
            Tile::SplitHorizontal if now.1 == Direction::Up || now.1 == Direction::Down => {
                if now.0 .1 > 0 {
                    let next_start = ((now.0 .0, now.0 .1 - 1), Direction::Left);
                    if !checked_start.contains(&next_start) {
                        start_rec(input, visited, next_start, checked_start);
                    }
                }
                if now.0 .1 < w - 1 {
                    let next_start = ((now.0 .0, now.0 .1 + 1), Direction::Right);
                    if !checked_start.contains(&next_start) {
                        start_rec(input, visited, next_start, checked_start);
                    }
                }
                break;
            }
            Tile::MirrorLDtoRT => match now.1 {
                Direction::Down => now.1 = Direction::Left,
                Direction::Up => now.1 = Direction::Right,
                Direction::Left => now.1 = Direction::Down,
                Direction::Right => now.1 = Direction::Up,
            },
            Tile::MirrorLTtoRD => match now.1 {
                Direction::Down => now.1 = Direction::Right,
                Direction::Up => now.1 = Direction::Left,
                Direction::Left => now.1 = Direction::Up,
                Direction::Right => now.1 = Direction::Down,
            },
            _ => {}
        }
        match now.1 {
            Direction::Up if now.0 .0 > 0 => {
                now.0 .0 -= 1;
            }
            Direction::Down if now.0 .0 < h - 1 => {
                now.0 .0 += 1;
            }
            Direction::Left if now.0 .1 > 0 => {
                now.0 .1 -= 1;
            }
            Direction::Right if now.0 .1 < w - 1 => {
                now.0 .1 += 1;
            }
            _ => {
                break;
            }
        }
    }
}

fn start_count(input: &Vec<Vec<Tile>>, start: ((usize, usize), Direction)) -> usize {
    let h = input.len();
    let w = input[0].len();
    let mut visited = vec![vec![false; w]; h];
    let mut checked_start = vec![];
    start_rec(input, &mut visited, start, &mut checked_start);
    visited.into_iter().flatten().filter(|b| *b).count()
}

fn compute_part2(input: Vec<Vec<Tile>>) -> usize {
    let h = input.len();
    let w = input[0].len();
    let top_row_max = (0..w)
        .map(|i| start_count(&input, ((0, i), Direction::Down)))
        .max()
        .unwrap();
    let bottom_row_max = (0..w)
        .map(|i| start_count(&input, ((h - 1, i), Direction::Up)))
        .max()
        .unwrap();
    let left_col_max = (0..h)
        .map(|i| start_count(&input, ((i, 0), Direction::Left)))
        .max()
        .unwrap();
    let right_col_max = (0..h)
        .map(|i| start_count(&input, ((i, w - 1), Direction::Right)))
        .max()
        .unwrap();
    vec![top_row_max, bottom_row_max, left_col_max, right_col_max]
        .into_iter()
        .max()
        .unwrap()
}

fn input() -> Vec<Vec<Tile>> {
    let string = std::fs::read_to_string("inout/day16.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Space,
                    '|' => Tile::SplitVertical,
                    '-' => Tile::SplitHorizontal,
                    '\\' => Tile::MirrorLTtoRD,
                    '/' => Tile::MirrorLDtoRT,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn read_str(string: &str) -> Vec<Vec<Tile>> {
        string
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|char| match char {
                        '.' => Tile::Space,
                        '|' => Tile::SplitVertical,
                        '-' => Tile::SplitHorizontal,
                        '\\' => Tile::MirrorLTtoRD,
                        '/' => Tile::MirrorLDtoRT,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
    #[test]
    fn start_rec_test() {
        let s = r".....";
        let i = read_str(s);
        assert_eq!(start_count(&i, ((0, 0), Direction::Right)), 5);

        let s = r"..|..";
        let i = read_str(s);
        assert_eq!(start_count(&i, ((0, 0), Direction::Right)), 3);

        let s = r"../..";
        let i = read_str(s);
        assert_eq!(start_count(&i, ((0, 0), Direction::Right)), 3);

        let s = r"..|..
                        ..-./";
        let i = read_str(s);
        assert_eq!(start_count(&i, ((0, 0), Direction::Right)), 9);

        let s = r"..|.\
                        ..-./";
        let i = read_str(s);
        assert_eq!(start_count(&i, ((0, 0), Direction::Right)), 10);
    }
}

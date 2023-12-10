use std::{
    collections::{HashSet, VecDeque},
    ops::Neg,
    str::FromStr,
};

fn get_pipe(pipes: &Vec<Vec<PipeInput>>, now: (usize, usize)) -> Option<PipeInput> {
    let now_line = pipes.get(now.0)?;
    now_line.get(now.1).cloned()
}

fn next_index(
    pipes: &Vec<Vec<PipeInput>>,
    now: (usize, usize),
    next: &Direction,
) -> Option<(usize, usize)> {
    let (h, w) = (pipes.len(), pipes[0].len());
    let now_pipe = get_pipe(pipes, now)?;
    if !now_pipe.has_direction(&next) {
        return None;
    }
    match next {
        Direction::N if now.0 > 0 => Some((now.0 - 1, now.1)),
        Direction::S if now.0 < h - 1 => Some((now.0 + 1, now.1)),
        Direction::W if now.1 > 0 => Some((now.0, now.1 - 1)),
        Direction::E if now.1 < w - 1 => Some((now.0, now.1 + 1)),
        _ => None,
    }
}

fn find_start(pipes: &Vec<Vec<PipeInput>>) -> (usize, usize) {
    for (i, line) in pipes.iter().enumerate() {
        for (j, pipe) in line.iter().enumerate() {
            if *pipe == PipeInput::Start {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn walk_from_start(
    pipes: &Vec<Vec<PipeInput>>,
    start: (usize, usize),
    start_direction: Direction,
) -> Option<Vec<Direction>> {
    let mut res: Vec<Direction> = vec![];
    let mut now = next_index(pipes, start, &start_direction)?;
    let mut from = start_direction;

    loop {
        let now_pipe = get_pipe(pipes, now)?;
        res.push(from.clone());
        match now_pipe {
            PipeInput::Nop => {
                return None;
            }
            PipeInput::Start => {
                break;
            }
            _ => {
                let next_direction = now_pipe.next_direction(&from)?;
                let next_index: (usize, usize) = next_index(pipes, now, &next_direction)?;
                now = next_index;
                from = next_direction;
            }
        }
    }

    Some(res)
}

fn compute_part1(pipes: &Vec<Vec<PipeInput>>) -> usize {
    let path = get_path(pipes);
    path.len() / 2
}

fn get_path(pipes: &Vec<Vec<PipeInput>>) -> Vec<Direction> {
    let start: (usize, usize) = find_start(pipes);
    for direction in vec![Direction::N, Direction::S, Direction::W, Direction::E] {
        let res = walk_from_start(pipes, start, direction);
        if let Some(v) = res {
            return v;
        }
    }
    unreachable!()
}

fn compute_part2(pipes: &Vec<Vec<PipeInput>>) -> usize {
    let path = get_path(pipes);
    let mut now = find_start(pipes);
    let mut sorted_path: Vec<(usize, usize)> = path
        .into_iter()
        .map(|direction| {
            now = next_index(pipes, now, &direction).unwrap();
            now.clone()
        })
        .collect::<Vec<_>>();

    sorted_path.sort();

    let h = pipes.len();
    let w = pipes[0].len();

    let mut components: Vec<HashSet<(usize, usize)>> = vec![];

    for i in 0..h {
        for j in 0..w {
            if components.iter().all(|set| !set.contains(&(i, j))) {
                let component_of_this = connected_component(h, w, &sorted_path, (i, j));
                if !component_of_this.is_empty() {
                    components.push(component_of_this);
                }
            }
        }
    }

    println!("{components:?}");

    components
        .into_iter()
        .filter(|set| {
            let pt_in_set: (usize, usize) = {
                let set: Vec<(usize, usize)> = Vec::from_iter(set.clone());
                set[0]
            };
            let path_to_outside = get_path_to_outside(h, w, &sorted_path, pt_in_set);
            let count = count_across(&sorted_path, &path_to_outside);
            count % 2 == 0
        })
        .map(|set| set.len())
        .sum()
}

fn connected_component(
    h: usize,
    w: usize,
    sorted_path: &Vec<(usize, usize)>,
    pt: (usize, usize),
) -> HashSet<(usize, usize)> {
    // path.sort();
    if sorted_path.binary_search(&pt).is_ok() {
        return HashSet::new();
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(pt);
    let mut area: HashSet<(usize, usize)> = HashSet::new();

    while let Some(next) = queue.pop_front() {
        // println!("{next:?}");
        if sorted_path.binary_search(&next).is_ok() {
            continue;
        }
        area.insert(next);
        if 0 < next.0 {
            let candidate = (next.0 - 1, next.1);
            if !queue.contains(&candidate) && !area.contains(&candidate) {
                queue.push_back(candidate);
            }
        }
        if next.0 < w - 1 {
            let candidate = (next.0 + 1, next.1);
            if !queue.contains(&candidate) && !area.contains(&candidate) {
                queue.push_back(candidate);
            }
        }
        if 0 < next.1 {
            let candidate = (next.0, next.1 - 1);
            if !queue.contains(&candidate) && !area.contains(&candidate) {
                queue.push_back(candidate);
            }
        }
        if next.1 < h - 1 {
            let candidate = (next.0, next.1 + 1);
            if !queue.contains(&candidate) && !area.contains(&candidate) {
                queue.push_back(candidate);
            }
        }
    }
    area
}

fn look_outside(h: usize, w: usize, sorted_path: &Vec<(usize, usize)>) -> (usize, usize) {
    for i in 0..h {
        if !connected_component(h, w, &sorted_path, (h, 0)).is_empty() {
            return (h, 0);
        }
    }
    unreachable!()
}

fn get_dir_from_index(from: (usize, usize), to: (usize, usize)) -> Option<Direction> {
    if from.0 == to.0 + 1 && from.1 == to.1 {
        Some(Direction::N)
    } else if from.0 + 1 == to.0 - 1 && from.1 == to.1 {
        Some(Direction::S)
    } else if from.0 == to.0 && from.1 == to.1 + 1 {
        Some(Direction::W)
    } else if from.0 == to.0 && from.1 == to.1 - 1 {
        Some(Direction::E)
    } else {
        None
    }
}

enum D {
    R,L,
}

fn count_across(loop_path: &Vec<(usize, usize)>, path: &Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    let mut prev: Option<(D, (usize, usize))> = None;
    for p in path {
        if loop_path.contains(p) {
            if let Some((d, _)) = prev {
                prev = Some((d, *p));
            } else {
                let loop_prev_index = {
                    let i = loop_path.iter().position(|pt| *pt == *p).unwrap();
                    if i > 0 {
                        i - 1
                    } else {
                        loop_path.len() - 1
                    }
                };
            }
        }
    }
    todo!()
}

fn get_path_to_outside(
    h: usize,
    w: usize,
    sorted_path: &Vec<(usize, usize)>,
    pt: (usize, usize),
) -> Vec<(usize, usize)> {
    let outside_pt = look_outside(h, w, sorted_path);
    let mut v = vec![];
    if outside_pt.0 < pt.0 {
        v.extend((outside_pt.0..=pt.0).map(|i| (i, outside_pt.1)));
    } else {
        v.extend((pt.0..=outside_pt.0).rev().map(|i| (i, outside_pt.1)));
    }
    if outside_pt.1 < pt.1 {
        v.extend((outside_pt.1..=pt.1).map(|i| (pt.0, i)));
    } else {
        v.extend((pt.1..=outside_pt.1).rev().map(|i| (pt.0, i)));
    }
    println!("{v:?}");
    v
}

fn main() {
    let pipes = input();
    // println!("{}", compute_part1(&pipes));
    println!("{}", compute_part2(&pipes));
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PipeInput {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
    Nop,
    Start,
}

impl PipeInput {
    fn has_direction(&self, direction: &Direction) -> bool {
        match (self, direction) {
            (PipeInput::Nop, _) => false,
            (PipeInput::Start, _) => true,
            (PipeInput::NS | PipeInput::NE | PipeInput::NW, Direction::N) => true,
            (PipeInput::NS | PipeInput::SE | PipeInput::SW, Direction::S) => true,
            (PipeInput::WE | PipeInput::NE | PipeInput::SE, Direction::E) => true,
            (PipeInput::WE | PipeInput::NW | PipeInput::SW, Direction::W) => true,
            _ => false,
        }
    }
    // this returns
    fn next_directions(&self, from: &Direction) -> Option<Vec<Direction>> {
        let from = from.clone().neg();
        if !self.has_direction(&from) {
            return None;
        }
        Some(
            vec![Direction::N, Direction::S, Direction::W, Direction::E]
                .into_iter()
                .filter(|direction| self.has_direction(direction) && *direction != from)
                .collect(),
        )
    }
    fn next_direction(&self, from: &Direction) -> Option<Direction> {
        let nexts = self.next_directions(from)?;
        if nexts.len() != 1 {
            None
        } else {
            Some(nexts[0].clone())
        }
    }
}

impl FromStr for PipeInput {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('|') => Ok(PipeInput::NS),
            Some('-') => Ok(PipeInput::WE),
            Some('L') => Ok(PipeInput::NE),
            Some('J') => Ok(PipeInput::NW),
            Some('7') => Ok(PipeInput::SW),
            Some('F') => Ok(PipeInput::SE),
            Some('.') => Ok(PipeInput::Nop),
            Some('S') => Ok(PipeInput::Start),
            _ => Err(s.to_string()),
        }
    }
}

fn input() -> Vec<Vec<PipeInput>> {
    let string = std::fs::read_to_string("inout/day10.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<PipeInput>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn pipe_and_direction_test() {
        let a = PipeInput::NS;
        let bs = vec![
            (Direction::N, Some(Direction::N)),
            (Direction::S, Some(Direction::S)),
            (Direction::E, None),
            (Direction::W, None),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }

        let a = PipeInput::WE;
        let bs = vec![
            (Direction::N, None),
            (Direction::S, None),
            (Direction::E, Some(Direction::E)),
            (Direction::W, Some(Direction::W)),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }

        let a = PipeInput::NE;
        let bs = vec![
            (Direction::N, None),
            (Direction::S, Some(Direction::E)),
            (Direction::E, None),
            (Direction::W, Some(Direction::N)),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }

        let a = PipeInput::SE;
        let bs = vec![
            (Direction::N, Some(Direction::E)),
            (Direction::S, None),
            (Direction::E, None),
            (Direction::W, Some(Direction::S)),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }

        let a = PipeInput::NW;
        let bs = vec![
            (Direction::N, None),
            (Direction::S, Some(Direction::W)),
            (Direction::E, Some(Direction::N)),
            (Direction::W, None),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }

        let a = PipeInput::SW;
        let bs = vec![
            (Direction::N, Some(Direction::W)),
            (Direction::S, None),
            (Direction::E, Some(Direction::S)),
            (Direction::W, None),
        ];
        for (b, e) in bs {
            assert_eq!(a.next_direction(&b), e);
        }
    }
    #[test]
    fn test() {
        let pipes = vec![
            vec![PipeInput::Start, PipeInput::SW],
            vec![PipeInput::NE, PipeInput::NW],
        ];
        let res = compute_part1(&pipes);
        assert_eq!(res, 2);
    }
    #[test]
    fn conn_comp_test() {
        let mut path = vec![
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (3, 2),
            (3, 1),
            (2, 1),
        ];
        path.sort();
        let v: HashSet<(usize, usize)> =
            HashSet::from_iter(connected_component(5, 5, &path, (2, 2)));
        assert_eq!(v, HashSet::from_iter(vec![(2, 2)]));

        let v: HashSet<(usize, usize)> =
            HashSet::from_iter(connected_component(5, 5, &path, (0, 0)));
        assert_eq!(
            v,
            HashSet::from_iter(vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 4),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
                (4, 2),
                (4, 1),
                (4, 0),
                (3, 0),
                (2, 0),
                (1, 0),
            ])
        );
    }
}

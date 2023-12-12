use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    ops::Neg,
    str::FromStr,
};

fn main() {
    let pipes = input();
    println!("{}", compute_part1(&pipes));
    println!("{}", compute_part2(&pipes));
}

fn compute_part1(pipes: &Vec<Vec<Option<PipeInput>>>) -> usize {
    let map = PipeMap::from_vec(pipes.clone());
    let path = map.get_path();
    path.len() / 2
}

struct PipeMap {
    start: (usize, usize),
    map: HashMap<(usize, usize), PipeInput>,
    height: usize,
    width: usize,
}

impl PipeMap {
    fn from_vec(v: Vec<Vec<Option<PipeInput>>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let mut map = HashMap::new();
        let mut start = (0, 0);
        for i in 0..height {
            for j in 0..width {
                if let Some(pipe) = v[i][j] {
                    if pipe == PipeInput::Special {
                        start = (i, j);
                    }
                    map.insert((i, j), pipe);
                }
            }
        }
        Self {
            start,
            map,
            height,
            width,
        }
    }
    fn height(&self) -> usize {
        self.height
    }
    fn width(&self) -> usize {
        self.width
    }
    fn get(&self, index: &(usize, usize)) -> Option<&PipeInput> {
        self.map.get(index)
    }
    fn checked_next_index(&self, now: (usize, usize), next: &Direction) -> Option<(usize, usize)> {
        let now_pipe = self.get(&now)?;
        if !now_pipe.has_direction(&next) {
            return None;
        }
        match next {
            Direction::N if now.0 > 0 => Some((now.0 - 1, now.1)),
            Direction::S if now.0 < self.height - 1 => Some((now.0 + 1, now.1)),
            Direction::W if now.1 > 0 => Some((now.0, now.1 - 1)),
            Direction::E if now.1 < self.width - 1 => Some((now.0, now.1 + 1)),
            _ => None,
        }
    }
    fn get_loop(&self) -> Loop {
        fn walk_from_start(
            map: &PipeMap,
            start_direction: Direction,
        ) -> Option<Vec<(usize, usize)>> {
            let mut res: Vec<(usize, usize)> = vec![map.start];
            let mut now = map.checked_next_index(map.start, &start_direction)?;
            let mut from_direction = start_direction;
            loop {
                res.push(now);
                let pipe = map.get(&now)?;
                match pipe {
                    PipeInput::Special => {
                        break;
                    }
                    _ => {
                        let next_direction = pipe.next_direction(&from_direction)?;
                        let next_index: (usize, usize) =
                            map.checked_next_index(now, &next_direction)?;
                        now = next_index;
                        from_direction = next_direction;
                    }
                }
            }
            Some(res)
        }
        for direction in vec![Direction::N, Direction::S, Direction::W, Direction::E] {
            if let Some(res) = walk_from_start(&self, direction) {
                return Loop::new(res);
            }
        }
        unreachable!()
    }
}

// struct of nice oriented path
// - path: all of points in vec is adjoin each other
// - pts is never return (i.e. there is no subseq such that ... pt1, pt2, pt1)
// => move of path can be represent as vec of PipeInput struct
struct PathNonDeg {
    path: Vec<(usize, usize)>,
    pipes: Vec<PipeInput>,
}

impl PathNonDeg {
    fn new(path: Vec<(usize, usize)>) -> Self {
        assert!(
            path.windows(2)
                // |(x0, x1) - (y0, y1) |_{Manhattan} = 1 <=> get_dir is some
                .all(|pts| get_dir_from_index(pts[0], pts[1]).is_some())
                && path.windows(3).all(|pts| pts[0] != pts[2])
        );
        let mut pipes = vec![PipeInput::Special];
        pipes.extend(path.windows(3).map(|pts| {
            let dir1 = get_dir_from_index(pts[0], pts[1]).unwrap();
            let dir2 = get_dir_from_index(pts[1], pts[2]).unwrap();
            match (dir1.neg(), dir2) {
                (Direction::N, Direction::S) | (Direction::S, Direction::N) => PipeInput::NS,
                (Direction::N, Direction::E) | (Direction::E, Direction::N) => PipeInput::NE,
                (Direction::N, Direction::W) | (Direction::W, Direction::N) => PipeInput::NW,
                (Direction::S, Direction::E) | (Direction::E, Direction::S) => PipeInput::SE,
                (Direction::S, Direction::W) | (Direction::W, Direction::S) => PipeInput::SW,
                (Direction::W, Direction::E) | (Direction::E, Direction::W) => PipeInput::WE,
                _ => unreachable!(),
            }
        }));
        pipes.push(PipeInput::Special);
        Self { path, pipes }
    }
    fn as_pipe(&self) -> &[PipeInput] {
        &self.pipes
    }
    // return which direction come from and go to
    fn dir_pipe_of_pt(&self, pt: (usize, usize)) -> Option<(Option<Direction>, Option<Direction>)> {
        let now_ind = self.path.iter().position(|pt_this| *pt_this == pt)?;
        let come_from = if now_ind == 0 {
            None
        } else {
            Some(get_dir_from_index(self.path[now_ind - 1], self.path[now_ind]).unwrap())
        };
        let go_to = if now_ind == self.path.len() - 1 {
            None
        } else {
            Some(get_dir_from_index(self.path[now_ind], self.path[now_ind + 1]).unwrap())
        };
        Some((come_from, go_to))
    }
    fn contains(&self, pt: (usize, usize)) -> bool {
        self.path
            .iter()
            .find(|pt_inthis| pt == **pt_inthis)
            .is_some()
    }
}

// struct of nice oriented loop
// - loop: all of points in vec is adjoin each other
// - pts is never return (i.e. there is no subseq such that ... pt1, pt2, pt1) as loop
// - so len should greater then 2
// => move of path can be represent as vec of PipeInput struct
struct Loop {
    path: Vec<(usize, usize)>,
    pipes: Vec<PipeInput>,
}

impl Loop {
    fn new(path: Vec<(usize, usize)>) -> Self {
        assert!(
            path.windows(2)
                // |(x0, x1) - (y0, y1) |_{Manhattan} = 1
                .all(|pts| get_dir_from_index(pts[0], pts[1]).is_some())
                && get_dir_from_index(path[path.len() - 1], path[0]).is_some()
                && path.windows(3).all(|pts| pts[0] != pts[2])
                && path.len() > 2
        );

        let mut pipes = vec![{
            let dir1 = get_dir_from_index(path[path.len() - 1], path[0]).unwrap();
            let dir2 = get_dir_from_index(path[0], path[1]).unwrap();
            match (dir1.neg(), dir2) {
                (Direction::N, Direction::S) | (Direction::S, Direction::N) => PipeInput::NS,
                (Direction::N, Direction::E) | (Direction::E, Direction::N) => PipeInput::NE,
                (Direction::N, Direction::W) | (Direction::W, Direction::N) => PipeInput::NW,
                (Direction::S, Direction::E) | (Direction::E, Direction::S) => PipeInput::SE,
                (Direction::S, Direction::W) | (Direction::W, Direction::S) => PipeInput::SW,
                (Direction::W, Direction::E) | (Direction::E, Direction::W) => PipeInput::WE,
                _ => unreachable!(),
            }
        }];
        pipes.extend(path.windows(3).map(|pts| {
            let dir1 = get_dir_from_index(pts[0], pts[1]).unwrap();
            let dir2 = get_dir_from_index(pts[1], pts[2]).unwrap();
            match (dir1.neg(), dir2) {
                (Direction::N, Direction::S) | (Direction::S, Direction::N) => PipeInput::NS,
                (Direction::N, Direction::E) | (Direction::E, Direction::N) => PipeInput::NE,
                (Direction::N, Direction::W) | (Direction::W, Direction::N) => PipeInput::NW,
                (Direction::S, Direction::E) | (Direction::E, Direction::S) => PipeInput::SE,
                (Direction::S, Direction::W) | (Direction::W, Direction::S) => PipeInput::SW,
                (Direction::W, Direction::E) | (Direction::E, Direction::W) => PipeInput::WE,
                _ => unreachable!(),
            }
        }));
        Self {
            path: path[..path.len() - 1].to_vec(),
            pipes,
        }
    }
    fn as_pipe(&self) -> &[PipeInput] {
        &self.pipes
    }
    // return which direction come from and go to
    fn dir_pipe_of_pt(&self, pt: (usize, usize)) -> Option<(Direction, Direction)> {
        let now_ind = self.path.iter().position(|pt_this| *pt_this == pt)?;
        let come_from = if now_ind == 0 {
            get_dir_from_index(self.path[self.path.len() - 1], self.path[0]).unwrap()
        } else {
            get_dir_from_index(self.path[now_ind - 1], self.path[now_ind]).unwrap()
        };
        let go_to = if now_ind == self.path.len() - 1 {
            get_dir_from_index(self.path[self.path.len() - 1], self.path[0]).unwrap()
        } else {
            get_dir_from_index(self.path[now_ind], self.path[now_ind + 1]).unwrap()
        };
        Some((come_from, go_to))
    }
    fn contains(&self, pt: (usize, usize)) -> bool {
        self.path
            .iter()
            .find(|pt_inthis| pt == **pt_inthis)
            .is_some()
    }
    fn len(&self) -> usize {
        self.path.len()
    }
}

enum Intersection {
    No,
    FromRight,
    FromLeft,
    AcrossRightToLeft,
    AcrossLeftToRight,
    TouchRight,
    TouchLeft,
    GoLeft,
    GoRight,
    Continue,
}

fn dir_intersection(dir1: (Direction, Direction), dir2: (Direction, Direction)) -> Intersection {
    match (dir1, dir2) {
        ((d11, d12), (d21, d22)) if d11 == d21 && d12 == d22 => Intersection::Continue,
        ((d11, d12), (d21, d22)) if d11 == d21 => {
            let d12 = d11.neg().rotate(d12);
            let d22 = d21.neg().rotate(d22);
            if d22 < d12 {
                Intersection::GoRight
            } else {
                Intersection::GoLeft
            }
        }
        ((d11, d12), (d21, d22)) if d12 == d22 => {
            let d11 = d12.rotate(d11.neg());
            let d21 = d22.rotate(d21.neg());
            if d21 < d11 {
                Intersection::FromRight
            } else {
                Intersection::FromLeft
            }
        }

        _ => unimplemented!(),
    }
}

fn count_across(loop_path: &Loop, path: &PathNonDeg) -> usize {
    assert!(
        !loop_path.contains(*path.path.first().unwrap())
            && !loop_path.contains(*path.path.last().unwrap())
    );
    let mut count = 0;
    let mut prev: Option<(D, (usize, usize))> = None;
    for pt in &path.path[1..path.path.len() - 1] {
        let Some((loop_come, loop_go)) = loop_path.dir_pipe_of_pt(*pt)else {
            continue;
        };

        let (Some(path_come), Some(path_go)) = path.dir_pipe_of_pt(*pt).unwrap() else {
            unreachable!()
        };
    }
    todo!()
}

fn compute_part2(pipes: &Vec<Vec<Option<PipeInput>>>) -> usize {
    let map = PipeMap::from_vec(pipes.clone());
    let loop_in_map = map.get_loop();
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

fn connected_component_of_pt(
    (h, w): (usize, usize),
    loop_in: &Loop,
    pt: (usize, usize),
) -> HashSet<(usize, usize)> {
    let sorted = {
        let mut p = loop_in.path.clone();
        p.sort();
        p
    };
    if sorted.binary_search(&pt).is_ok() {
        return HashSet::new();
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(pt);
    let mut area: HashSet<(usize, usize)> = HashSet::new();

    while let Some(next) = queue.pop_front() {
        // println!("{next:?}");
        if sorted.binary_search(&next).is_ok() {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Direction {
    N,
    E,
    S,
    W,
}
// nwse
impl Direction {
    // Orientation of the other when the coordinates are rotated so that self is north.
    fn rotate(self, other: Direction) -> Direction {
        match (self, other) {
            (Direction::N, _) => other,

            (Direction::E, Direction::N) => Direction::W,
            (Direction::E, Direction::E) => Direction::N,
            (Direction::E, Direction::S) => Direction::E,
            (Direction::E, Direction::W) => Direction::S,

            (Direction::W, Direction::N) => Direction::E,
            (Direction::W, Direction::E) => Direction::S,
            (Direction::W, Direction::S) => Direction::W,
            (Direction::W, Direction::W) => Direction::N,

            (Direction::S, Direction::N) => Direction::S,
            (Direction::S, Direction::E) => Direction::W,
            (Direction::S, Direction::S) => Direction::N,
            (Direction::S, Direction::W) => Direction::E,
        }
    }
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
    Special,
}

impl PipeInput {
    fn has_direction(&self, direction: &Direction) -> bool {
        match (self, direction) {
            (PipeInput::Special, _) => true,
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

fn from_char(char: char) -> Option<Option<PipeInput>> {
    match char {
        '|' => Some(Some(PipeInput::NS)),
        '-' => Some(Some(PipeInput::WE)),
        'L' => Some(Some(PipeInput::NE)),
        'J' => Some(Some(PipeInput::NW)),
        '7' => Some(Some(PipeInput::SW)),
        'F' => Some(Some(PipeInput::SE)),
        '.' => Some(None),
        'S' => Some(Some(PipeInput::Special)),
        _ => None,
    }
}

fn input() -> Vec<Vec<Option<PipeInput>>> {
    let string = std::fs::read_to_string("inout/day10.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| from_char(char).unwrap())
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
            vec![PipeInput::Special, PipeInput::SW]
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>(),
            vec![PipeInput::NE, PipeInput::NW]
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>(),
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

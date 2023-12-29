use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

fn main() {
    let input = input();
    println!("{}", compute_part1(&input));
    println!("{}", compute_part2(&input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn get_len_with_dir(
    pos0: (usize, usize, Direction),
    pos1: (usize, usize, Direction),
) -> Option<usize> {
    match pos0.2 {
        Direction::Down
            if pos0.0 <= pos1.0
                && pos0.1 == pos1.1
                && (pos1.2 == Direction::Right || pos1.2 == Direction::Left) =>
        {
            Some(pos1.0 - pos0.0)
        }
        Direction::Up
            if pos1.0 <= pos0.0
                && pos0.1 == pos1.1
                && (pos1.2 == Direction::Right || pos1.2 == Direction::Left) =>
        {
            Some(pos0.0 - pos1.0)
        }
        Direction::Right
            if pos0.1 <= pos1.1
                && pos0.0 == pos1.0
                && (pos1.2 == Direction::Up || pos1.2 == Direction::Down) =>
        {
            Some(pos1.1 - pos0.1)
        }
        Direction::Left
            if pos1.1 <= pos0.1
                && pos0.0 == pos1.0
                && (pos1.2 == Direction::Up || pos1.2 == Direction::Down) =>
        {
            Some(pos0.1 - pos1.1)
        }
        _ => None,
    }
}

type Node = (usize, usize, Direction);

fn compute_adjacence_cost(input: &Vec<Vec<usize>>, node1: &Node, node2: &Node) -> Option<usize> {
    if let Some(len) = get_len_with_dir(node1.clone(), node2.clone()) {
        match node1.2 {
            Direction::Down => {
                return Some((node1.0 + 1..=node2.0).map(|i| input[i][node1.1]).sum());
            }
            Direction::Up => {
                return Some((node2.0..node1.0).map(|i| input[i][node1.1]).sum());
            }
            Direction::Right => {
                return Some((node1.1 + 1..=node2.1).map(|i| input[node1.0][i]).sum());
            }
            Direction::Left => {
                return Some((node2.1..node1.1).map(|i| input[node1.0][i]).sum());
            }
        }
    }
    None
}

fn enumerate_adjacence(
    (h, w): (usize, usize),
    node: &Node,
    min_len: usize,
    max_len: usize,
) -> Vec<Node> {
    match node.2 {
        Direction::Down => (min_len..=max_len)
            .flat_map(|i| {
                if node.0 + i < h {
                    Some(vec![
                        (node.0 + i, node.1, Direction::Right),
                        (node.0 + i, node.1, Direction::Left),
                    ])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>(),
        Direction::Up => (min_len..=max_len)
            .flat_map(|i| {
                if i <= node.0 {
                    Some(vec![
                        (node.0 - i, node.1, Direction::Right),
                        (node.0 - i, node.1, Direction::Left),
                    ])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>(),
        Direction::Right => (min_len..=max_len)
            .flat_map(|i| {
                if node.1 + i < w {
                    Some(vec![
                        (node.0, node.1 + i, Direction::Up),
                        (node.0, node.1 + i, Direction::Down),
                    ])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>(),
        Direction::Left => (min_len..=max_len)
            .flat_map(|i| {
                if i <= node.1 {
                    Some(vec![
                        (node.0, node.1 - i, Direction::Up),
                        (node.0, node.1 - i, Direction::Down),
                    ])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>(),
    }
}

fn cost_path(input: &Vec<Vec<usize>>, path: Vec<Node>) -> usize {
    path.windows(2)
        .map(|nodes| {
            let [node1, node2,..] = nodes else {
                unreachable!()
            };
            let cost = compute_adjacence_cost(input, node1, node2).unwrap();
            cost
        })
        .sum()
}

fn compute_part1(input: &Vec<Vec<usize>>) -> usize {
    // vertex of graph = {(i,j,d) | 0 <= i < h, 0 <= j < w, d \in {U,D,R,L}}
    // edge of graph = { (i1,j1,d1), (i2,j2,d2) \in (vertex, vertex) | (i2, j2) is in d1 direction from (i1, j1) and dist <= 3 && direction is nice }
    type Node = (usize, usize, Direction);

    let h = input.len();
    let w = input[0].len();
    let huristic_to_goal = |node: &Node| h - 1 - node.0 + w - 1 - node.1;

    let adjacence_cost = |node1: &Node, node2: &Node| -> Option<usize> {
        compute_adjacence_cost(input, node1, node2)
    };

    let enumerate_adjacence =
        |node: &Node| -> Vec<Node> { enumerate_adjacence((h, w), node, 1, 3) };

    let goals: HashSet<(usize, usize, Direction)> = HashSet::from_iter(vec![
        (h - 1, w - 1, Direction::Down),
        (h - 1, w - 1, Direction::Up),
        (h - 1, w - 1, Direction::Right),
        (h - 1, w - 1, Direction::Left),
    ]);

    let may_path1 = a_star_algorithm(
        (0, 0, Direction::Right),
        goals.clone(),
        adjacence_cost,
        enumerate_adjacence,
        huristic_to_goal,
    );

    let may_path2 = a_star_algorithm(
        (0, 0, Direction::Down),
        goals,
        adjacence_cost,
        enumerate_adjacence,
        huristic_to_goal,
    );

    let cost_path = |path: Vec<Node>| -> usize { cost_path(input, path) };

    match (may_path1, may_path2) {
        (Some(mut path1), Some(mut path2)) => {
            path1.reverse();
            path2.reverse();
            std::cmp::min(cost_path(path1), cost_path(path2))
        }
        (Some(mut path), None) | (None, Some(mut path)) => {
            path.reverse();
            cost_path(path)
        }
        (None, None) => unreachable!("path not found"),
    }
}

fn compute_part2(input: &Vec<Vec<usize>>) -> usize {
    // vertex of graph = {(i,j,d) | 0 <= i < h, 0 <= j < w, d \in {U,D,R,L}}
    // edge of graph = { (i1,j1,d1), (i2,j2,d2) \in (vertex, vertex) | (i2, j2) is in d1 direction from (i1, j1) and dist <= 3 && direction is nice }
    type Node = (usize, usize, Direction);

    let h = input.len();
    let w = input[0].len();
    let huristic_to_goal = |node: &Node| h - 1 - node.0 + w - 1 - node.1;

    let adjacence_cost = |node1: &Node, node2: &Node| -> Option<usize> {
        compute_adjacence_cost(input, node1, node2)
    };

    let enumerate_adjacence =
        |node: &Node| -> Vec<Node> { enumerate_adjacence((h, w), node, 4, 10) };

    let goals: HashSet<(usize, usize, Direction)> = HashSet::from_iter(vec![
        (h - 1, w - 1, Direction::Down),
        (h - 1, w - 1, Direction::Up),
        (h - 1, w - 1, Direction::Right),
        (h - 1, w - 1, Direction::Left),
    ]);

    let may_path1 = a_star_algorithm(
        (0, 0, Direction::Right),
        goals.clone(),
        adjacence_cost,
        enumerate_adjacence,
        huristic_to_goal,
    );

    let may_path2 = a_star_algorithm(
        (0, 0, Direction::Down),
        goals,
        adjacence_cost,
        enumerate_adjacence,
        huristic_to_goal,
    );

    let cost_path = |path: Vec<Node>| -> usize { cost_path(input, path) };

    match (may_path1, may_path2) {
        (Some(mut path1), Some(mut path2)) => {
            path1.reverse();
            path2.reverse();
            std::cmp::min(cost_path(path1), cost_path(path2))
        }
        (Some(mut path), None) | (None, Some(mut path)) => {
            path.reverse();
            cost_path(path)
        }
        (None, None) => unreachable!("path not found"),
    }
}

fn a_star_algorithm<Node>(
    start: Node,
    goal: HashSet<Node>,
    adjacence_cost_fn: impl Fn(&Node, &Node) -> Option<usize>,
    enumerate_adjacence: impl Fn(&Node) -> Vec<Node>,
    huristic_to_goal: impl Fn(&Node) -> usize,
) -> Option<Vec<Node>>
where
    Node: Clone + PartialEq + Eq + Ord + Hash + Debug,
{
    let mut open_set: BinaryHeap<(Reverse<usize>, Node)> = BinaryHeap::new();
    open_set.push((Reverse(0), start.clone()));

    let mut come_from: HashMap<Node, Node> = HashMap::new();

    let mut known_g_score: HashMap<Node, usize> = HashMap::new();
    known_g_score.insert(start.clone(), 0);

    let mut current_guess: HashMap<Node, usize> = HashMap::new();
    current_guess.insert(start.clone(), huristic_to_goal(&start));

    let mut count = 0;
    while let Some((_, mut current)) = open_set.pop() {
        if goal.contains(&current) {
            let mut total_path = vec![current.clone()];
            while let Some(come_from) = come_from.get(&current) {
                current = come_from.clone();
                total_path.push(current.clone());
            }
            return Some(total_path);
        }
        for adjacence in enumerate_adjacence(&current) {
            let tentatice_g_score = *known_g_score.get(&current).unwrap()
                + adjacence_cost_fn(&current, &adjacence).unwrap();
            if known_g_score.get(&adjacence).is_none()
                || tentatice_g_score < *known_g_score.get(&adjacence).unwrap()
            {
                come_from.insert(adjacence.clone(), current.clone());
                known_g_score.insert(adjacence.clone(), tentatice_g_score);
                current_guess.insert(
                    adjacence.clone(),
                    tentatice_g_score + huristic_to_goal(&adjacence),
                );
                if open_set.iter().all(|node| node.1 != adjacence) {
                    open_set.push((
                        Reverse(tentatice_g_score + huristic_to_goal(&adjacence)),
                        adjacence,
                    ));
                }
            }
        }
    }
    None
}

fn input() -> Vec<Vec<usize>> {
    let string = std::fs::read_to_string("inout/day17.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| match char.to_digit(10) {
                    Some(digit) => digit as usize,
                    None => unreachable!("{}", char),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_star_algorithm_test() {
        let edges: HashMap<(usize, usize), usize> = HashMap::from_iter(vec![
            ((0, 1), 2),
            ((0, 2), 1),
            ((1, 3), 3),
            ((1, 4), 2),
            ((2, 4), 1),
            ((3, 5), 1),
        ]);
        let ad_cost = |i: &usize, j: &usize| -> Option<usize> { edges.get(&(*i, *j)).cloned() };
        let enu = |i: &usize| {
            edges
                .iter()
                .filter_map(|((from, to), _cost)| if from == i { Some(*to) } else { None })
                .collect::<Vec<_>>()
        };

        let huristic: HashMap<usize, usize> =
            HashMap::from_iter(vec![(0, 3), (1, 2), (2, 4), (3, 1), (4, 2), (5, 0)]);
        let hu_cost = |i: &usize| *huristic.get(i).unwrap();
        let path = a_star_algorithm(0, HashSet::from_iter(vec![5]), ad_cost, enu, hu_cost);
        assert_eq!(path, Some(vec![5, 3, 1, 0]));
    }
    #[test]
    fn example_case() {
        let input = vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ];
        assert_eq!(
            compute_adjacence_cost(&input, &(0, 0, Direction::Right), &(0, 2, Direction::Down)),
            Some(5)
        );
        assert_eq!(
            compute_adjacence_cost(&input, &(0, 8, Direction::Down), &(2, 8, Direction::Right)),
            Some(8)
        );
        assert_eq!(
            compute_adjacence_cost(&input, &(0, 8, Direction::Down), &(3, 8, Direction::Right)),
            Some(12)
        );
        assert_eq!(
            enumerate_adjacence((13, 13), &(0, 2, Direction::Down), 1, 3),
            vec![
                (1, 2, Direction::Right),
                (1, 2, Direction::Left),
                (2, 2, Direction::Right),
                (2, 2, Direction::Left),
                (3, 2, Direction::Right),
                (3, 2, Direction::Left),
            ]
        );
        assert_eq!(
            enumerate_adjacence((13, 13), &(1, 5, Direction::Up), 1, 3),
            vec![(0, 5, Direction::Right), (0, 5, Direction::Left),]
        );
        let path = vec![
            (0, 0, Direction::Right),
            (0, 2, Direction::Down),
            (1, 2, Direction::Right),
            (1, 5, Direction::Up),
            (0, 5, Direction::Right),
            (0, 8, Direction::Down),
            (2, 8, Direction::Right),
            (2, 10, Direction::Down),
            (4, 10, Direction::Right),
            (4, 11, Direction::Down),
            (7, 11, Direction::Right),
            (7, 12, Direction::Down),
            (10, 12, Direction::Left),
            (10, 11, Direction::Down),
            (12, 11, Direction::Right),
            (12, 12, Direction::Up),
        ];
        assert_eq!(cost_path(&input, path), 102);
        assert_eq!(compute_part1(&input), 102)
    }
}

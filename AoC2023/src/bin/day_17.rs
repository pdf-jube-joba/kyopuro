use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

fn main() {
    let i = input();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn compute_part1(input: &Vec<Vec<usize>>) -> usize {
    // vertex of graph = {(i,j,d) | 0 <= i < h, 0 <= j < w, d \in {U,D,R,L}}
    // edge of graph = { (i1,j1,d1), (i2,j2,d2) \in (vertex, vertex) | (i2, j2) is in d1 direction from (i1, j1) and dist <= 3 && direction is nice }
    type Node = (usize, usize, Direction);

    let h = input.len();
    let w = input[0].len();
    let huristic_to_goal = |node: &Node| h - node.0 + w - node.1;

    let adjacence_cost_fn = |node1: &Node, node2: &Node| -> Option<usize> {
        match node1.2 {
            Direction::Down
                if node1.0 < node2.0
                    && node2.0 < node1.0 + 3
                    && node1.1 == node2.1
                    && (node2.2 == Direction::Left || node2.2 == Direction::Right) =>
            {
                Some((node1.0 + 1..=node2.0).map(|i| input[i][node1.1]).sum())
            }
            Direction::Up
                if node2.0 < node1.0
                    && node1.0 < node2.0 + 3
                    && node1.1 == node2.1
                    && (node2.2 == Direction::Left || node2.2 == Direction::Right) =>
            {
                Some((node2.0 + 1..=node1.0).map(|i| input[i][node1.1]).sum())
            }
            Direction::Right
                if node1.1 < node2.1
                    && node2.1 < node1.1 + 3
                    && node1.0 == node2.0
                    && (node2.2 == Direction::Down || node2.2 == Direction::Up) =>
            {
                Some((node1.1 + 1..=node2.1).map(|i| input[node1.0][i]).sum())
            }
            Direction::Left
                if node2.1 < node1.1
                    && node1.1 < node2.1 + 3
                    && node1.0 == node2.0
                    && (node2.2 == Direction::Down || node2.2 == Direction::Up) =>
            {
                Some((node2.1 + 1..=node1.1).map(|i| input[node1.0][i]).sum())
            }
            _ => None,
        }
    };

    let enumerate_adjacence = |node: &Node| -> Vec<Node> { match node.2 {
        Direction::Down => {
            vec![
                if node.0 + 1 < h {Some((node.0 + 1, node.1, Direction::Right))} else {None},
                if node.0 + 2 < h {Some((node.0 + 2, node.1, Direction::Right))} else {None},
                if node.0 + 3 < h {Some((node.0 + 3, node.1, Direction::Right))} else {None},
                if node.0 + 1 < h {Some((node.0 + 1, node.1, Direction::Left))} else {None},
                if node.0 + 2 < h {Some((node.0 + 2, node.1, Direction::Left))} else {None},
                if node.0 + 3 < h {Some((node.0 + 3, node.1, Direction::Left))} else {None},
            ].into_iter().flatten().collect::<Vec<_>>()
        }
        Direction::Up => {
            vec![
                if 0 < node.0 {Some((node.0 - 1, node.1, Direction::Right))} else {None},
                if 1 < node.0 {Some((node.0 - 2, node.1, Direction::Right))} else {None},
                if 2 < node.0 {Some((node.0 - 3, node.1, Direction::Right))} else {None},
                if 0 < node.0 {Some((node.0 - 1, node.1, Direction::Left))} else {None},
                if 1 < node.0 {Some((node.0 - 2, node.1, Direction::Left))} else {None},
                if 2 < node.0 {Some((node.0 - 3, node.1, Direction::Left))} else {None},
            ].into_iter().flatten().collect::<Vec<_>>()
        }
        Direction::Right => {
            vec![
                if node.1 + 1 < w {Some((node.0, node.1 + 1, Direction::Up))} else {None},
                if node.1 + 2 < w {Some((node.0, node.1 + 2, Direction::Up))} else {None},
                if node.1 + 3 < w {Some((node.0, node.1 + 3, Direction::Up))} else {None},
                if node.1 + 1 < w {Some((node.0, node.1 + 1, Direction::Down))} else {None},
                if node.1 + 2 < w {Some((node.0, node.1 + 2, Direction::Down))} else {None},
                if node.1 + 3 < w {Some((node.0, node.1 + 3, Direction::Down))} else {None},
                ].into_iter().flatten().collect::<Vec<_>>()
        }
        Direction::Left => {
            vec![
                if 0 < node.1 {Some((node.0, node.1 - 1, Direction::Up))} else {None},
                if 1 < node.1 {Some((node.0, node.1 - 2, Direction::Up))} else {None},
                if 2 < node.1 {Some((node.0, node.1 - 3, Direction::Up))} else {None},
                if 0 < node.1 {Some((node.0, node.1 - 1, Direction::Down))} else {None},
                if 1 < node.1 {Some((node.0, node.1 - 2, Direction::Down))} else {None},
                if 2 < node.1 {Some((node.0, node.1 - 3, Direction::Down))} else {None},
            ].into_iter().flatten().collect::<Vec<_>>()
        }
    } };

    let may_path1 = a_star_algorithm(
        (0, 0, Direction::Right),
        HashSet::from_iter(vec![
            (h - 1, w - 1, Direction::Down),
            (h - 1, w - 1, Direction::Right),
        ]),
        adjacence_cost_fn,
        enumerate_adjacence,
        huristic_to_goal,
    );
    let may_path2 = a_star_algorithm(
        (0, 0, Direction::Down),
        HashSet::from_iter(vec![
            (h - 1, w - 1, Direction::Down),
            (h - 1, w - 1, Direction::Right),
        ]),
        adjacence_cost_fn,
        enumerate_adjacence,
        huristic_to_goal,
    );

    let cost_path = |path: Vec<Node>| -> usize {
        path.windows(2)
            .map(|nodes| {
                let [node1, node2,..] = nodes else {
                unreachable!()
            };
                adjacence_cost_fn(&node1, &node2).unwrap()
            })
            .sum()
    };

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

    while let Some((_, mut current)) = open_set.pop() {
        println!("{current:?}");
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
}

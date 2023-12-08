use crate::nums::*;

fn step_num(
    moves: &Vec<Move>,
    sorted_maps: &Vec<(OneNode, OneNode, OneNode)>,
    mut now: OneNode,
    is_goal: impl Fn(&OneNode) -> bool,
) -> usize {
    for (step, move_lr) in moves.iter().cycle().enumerate() {
        if is_goal(&now) {
            return step;
        }
        let i = sorted_maps
            .binary_search_by(|(node_name, _, _)| node_name.cmp(&now))
            .unwrap(); // ok?
        now = match move_lr {
            Move::L => sorted_maps[i].1.clone(),
            Move::R => sorted_maps[i].2.clone(),
        };
    }
    unreachable!()
}

fn compute_part1(moves: Vec<Move>, mut maps: Vec<(OneNode, OneNode, OneNode)>) -> usize {
    maps.sort_by_key(|(node, _, _)| node.clone());
    step_num(&moves, &maps, OneNode::start(), |node| node.is_end())
}

fn compute_part2(moves: Vec<Move>, mut maps: Vec<(OneNode, OneNode, OneNode)>) -> usize {
    maps.sort_by_key(|(node, _, _)| node.clone());
    maps.iter()
        .filter(|(node, _, _)| node.is_start_with_a())
        .map(|(node, _, _)| node)
        .cloned()
        .map(|start_node| step_num(&moves, &maps, start_node, |node| node.is_end_with_z()))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    let input = input();
    println!("{}", compute_part1(input.0.clone(), input.1.clone()));
    println!("{}", compute_part2(input.0.clone(), input.1.clone()));
}

#[derive(Debug, Clone, PartialEq)]
enum Move {
    L,
    R,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct OneNode(String);

impl OneNode {
    fn new(str: String) -> Self {
        let chars = str.chars().collect::<Vec<_>>();
        debug_assert!(chars.len() == 3);
        debug_assert!(chars
            .iter()
            .all(|char| char.is_ascii_alphabetic() && char.is_uppercase()));
        Self(str)
    }
    fn is_start_with_a(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_end_with_z(&self) -> bool {
        self.0.ends_with('Z')
    }
    fn start() -> Self {
        OneNode("AAA".to_owned())
    }
    fn is_end(&self) -> bool {
        *self == OneNode("ZZZ".to_owned())
    }
}

fn input() -> (Vec<Move>, Vec<(OneNode, OneNode, OneNode)>) {
    let string = std::fs::read_to_string("inout/day8.in").unwrap();
    let mut lines = string.lines();
    let moves = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'R' => Move::R,
            'L' => Move::L,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let _ = lines.next().unwrap();

    let node_infos = lines
        .map(|line| {
            let node_name = line[..line.find("=").unwrap()].trim().to_owned();
            let left_node = line[line.find("(").unwrap() + 1..line.find(",").unwrap()]
                .trim()
                .to_owned();
            let right_node = line[line.find(",").unwrap() + 1..line.find(")").unwrap()]
                .trim()
                .to_owned();
            (
                OneNode::new(node_name),
                OneNode::new(left_node),
                OneNode::new(right_node),
            )
        })
        .collect::<Vec<_>>();

    (moves, node_infos)
}

mod nums {
    pub fn lcm(mut a: usize, mut b: usize) -> usize {
        if gcd(a, b) == 0 {
            0
        } else {
            (a * b) / gcd(a, b)
        }
    }
    fn gcd(mut a: usize, mut b: usize) -> usize {
        if a > b {
            gcd(b, a)
        // a <= b ?
        } else if a == 0 {
            0
        } else if b % a == 0 {
            a
        } else {
            gcd(b % a, a)
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn gcd_test() {
            assert_eq!(gcd(1, 1), 1);
            assert_eq!(gcd(2, 3), 1);
            assert_eq!(gcd(3, 2), 1);
            assert_eq!(gcd(2, 2), 2);
            assert_eq!(gcd(4, 6), 2);
        }
        #[test]
        fn lcm_test() {
            assert_eq!(lcm(1, 1), 1);
            assert_eq!(lcm(2, 3), 6);
            assert_eq!(lcm(3, 2), 6);
            assert_eq!(lcm(2, 2), 2);
            assert_eq!(lcm(4, 6), 12);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_test() {
        let mut a = (
            vec![Move::R, Move::L],
            vec![
                ("AAA", "BBB", "CCC"),
                ("BBB", "DDD", "EEE"),
                ("CCC", "ZZZ", "GGG"),
                ("DDD", "DDD", "DDD"),
                ("EEE", "EEE", "EEE"),
                ("GGG", "GGG", "GGG"),
                ("ZZZ", "ZZZ", "ZZZ"),
            ]
            .into_iter()
            .map(|(v0, v1, v2)| {
                (
                    OneNode::new(v0.to_owned()),
                    OneNode::new(v1.to_owned()),
                    OneNode::new(v2.to_owned()),
                )
            })
            .collect::<Vec<_>>(),
        );
        assert_eq!(compute_part1(a.0, a.1), 2);
    }
}

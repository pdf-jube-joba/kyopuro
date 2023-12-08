use crate::nums::*;

fn compute_part1(input: &(Vec<Move>, Vec<(OneNode, OneNode, OneNode)>)) -> usize {
    let mut nodes_network = input.1.to_owned();
    nodes_network.sort();

    let mut now = OneNode::new("AAA".to_owned());
    let goal = OneNode::new("ZZZ".to_owned());

    for (i, move_lr) in input.0.iter().cycle().enumerate() {
        if now == goal {
            return i;
        }
        let i = nodes_network
            .binary_search_by(|(node_name, _, _)| node_name.cmp(&now))
            .unwrap(); // ok?
        let (l, r) = (nodes_network[i].1.clone(), nodes_network[i].2.clone());
        now = match move_lr {
            Move::L => l,
            Move::R => r,
        };
    }
    unreachable!()
}

fn compute_part2(input: &(Vec<Move>, Vec<(OneNode, OneNode, OneNode)>)) -> usize {
    let mut nodes_network = input.1.to_owned();
    nodes_network.sort();
    let mut periods: Vec<usize> = vec![];
    let mut nows = nodes_network
        .iter()
        .filter_map(|(node, _, _)| {
            if node.is_start() {
                Some(node.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut nexts = vec![];
    for (i, move_lr) in input.0.iter().cycle().enumerate() {
        if nows.is_empty() {
            break;
        }
        for now in nows {
            if now.is_end() {
                periods.push(i);
                continue;
            }

            let i = nodes_network
                .binary_search_by(|(node_name, _, _)| node_name.cmp(&now))
                .unwrap(); // ok?
            let (l, r) = (nodes_network[i].1.clone(), nodes_network[i].2.clone());
            let next = match move_lr {
                Move::L => l,
                Move::R => r,
            };
            nexts.push(next);
        }
        nows = nexts;
        nexts = vec![];
    }
    lcms(periods)
}

fn main() {
    let input = input();
    println!("{}", compute_part1(&input));
    println!("{}", compute_part2(&input));
}

mod nums {
    pub fn lcms(mut a: Vec<usize>) -> usize {
        if a.is_empty() {
            panic!();
        }
        loop {
            if a.len() == 1 {
                return a[0];
            }
            let lcm_two = lcm(a.pop().unwrap(), a.pop().unwrap());
            a.push(lcm_two);
        }
    }
    fn lcm(mut a: usize, mut b: usize) -> usize {
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
            assert_eq!(lcms(vec![1, 1]), 1);
            assert_eq!(lcms(vec![2, 4, 6]), 12);
            assert_eq!(lcms(vec![3, 5, 7]), 3 * 5 * 7);
            assert_eq!(lcm(2 * 3, 2 * 3 * 3), 2 * 3 * 3);
            assert_eq!(lcm(2 * 5, 2 * 3 * 3 * 7), 2 * 3 * 3 * 5 * 7);
            assert_eq!(
                lcms(vec![2 * 3, 2 * 5, 2 * 7, 2 * 3 * 3]),
                2 * 3 * 3 * 5 * 7
            );
        }
    }
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
    fn is_start(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_end(&self) -> bool {
        self.0.ends_with('Z')
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_test() {
        let a = (
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
                    OneNode::new(v1.to_owned()),
                )
            })
            .collect::<Vec<_>>(),
        );
        assert_eq!(compute_part1(&a), 2)
    }
}

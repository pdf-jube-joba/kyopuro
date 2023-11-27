struct InfoChar {
    c: char,
    frequency: usize,
    depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Leaf(Leaf),
    Tree(Tree),
}

impl Node {
    fn leaf(c: char, frequency: usize) -> Self {
        Node::Leaf(Leaf::new(c, frequency))
    }
    fn tree(left: Node, right: Node) -> Self {
        Node::Tree(Tree::new(left, right))
    }
    fn frequency(&self) -> usize {
        match self {
            Node::Leaf(l) => l.frequency(),
            Node::Tree(t) => t.frequency(),
        }
    }
    fn collect_info_of_char(&self) -> Vec<InfoChar> {
        match &self {
            Node::Leaf(leaf) => vec![leaf.collect_info_of_char()],
            Node::Tree(tree) => tree.collect_info_of_char(),
        }
    }
    fn height(&self) -> usize {
        match &self {
            Node::Leaf(_) => 1,
            Node::Tree(tree) => tree.height(),
        }
    }
}

use std::cmp::{Ord, Ordering, PartialOrd};
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Node::Leaf(l1), Node::Leaf(l2)) => l1.cmp(&l2),
            (Node::Tree(t1), Node::Tree(t2)) => t1.cmp(&t2),
            // if frequency is equal, leaf is smaller then tree
            (Node::Leaf(l), Node::Tree(t)) => l.frequency.cmp(&t.frequency).then(Ordering::Less),
            (Node::Tree(t), Node::Leaf(l)) => l.frequency.cmp(&t.frequency).then(Ordering::Greater),
        })
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tree {
    frequency: usize,
    left: Box<Node>,
    right: Box<Node>,
}

impl Tree {
    fn new(left: Node, right: Node) -> Self {
        let frequency = &left.frequency()
            + &right.frequency();
        Self {
            frequency,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    fn collect_info_of_char(&self) -> Vec<InfoChar> {
        let mut v = vec![];
        let left_infos = self
            .left
            .as_ref()
            .collect_info_of_char();
        v.extend(left_infos);
        let right_infos = self
            .left
            .as_ref()
            .collect_info_of_char();
        v.extend(right_infos);
        v
    }
    fn height(&self) -> usize {
        let right = self.right.as_ref().height();
        let left = self.left.as_ref().height();
        std::cmp::max(right, left)
    }
    fn frequency(&self) -> usize {
        self.frequency
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.frequency
                .cmp(&other.frequency)
                .then(self.left.cmp(&other.left))
                .then(self.right.cmp(&other.right)),
        )
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Leaf {
    c: char,
    frequency: usize,
}

impl Leaf {
    fn new(c: char, frequency: usize) -> Self {
        Self { c, frequency }
    }
    fn collect_info_of_char(&self) -> InfoChar {
        InfoChar {
            c: self.c,
            frequency: self.frequency,
            depth: 1,
        }
    }
    fn frequency(&self) -> usize {
        self.frequency
    }
}

impl PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.frequency
                .cmp(&other.frequency)
                .then(self.c.cmp(&other.c)),
        )
    }
}

impl Ord for Leaf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn construct_Huffman_tree(str: &[char]) -> Option<Node> {
    use std::collections::{HashMap, BinaryHeap};
    use std::cmp::Reverse;
    let mut count = HashMap::new();
    for c in str {
        count.entry(*c).and_modify(|count| *count += 1).or_insert(1);
    }
    // min heap of node by frequency
    let mut freq_priority_queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    for (c, frequency) in count {
        let node = Node::leaf(c, frequency);
        freq_priority_queue.push(Reverse(node));
    }

    while freq_priority_queue.len() > 1 {
        let node1 = freq_priority_queue.pop().unwrap().0;
        let node2 = freq_priority_queue.pop().unwrap().0;
        let node_tree = Node::tree(node1, node2);
        freq_priority_queue.push(Reverse(node_tree));
    }
    freq_priority_queue.pop().map(|rev_node| rev_node.0)
}

fn compute_total_len(v: Vec<InfoChar>) -> usize {
    v.into_iter().map(|info| info.frequency * info.depth).sum()
}

fn main() {
    let str = input();
}

fn input() -> Vec<char> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    buf.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_test() {
        let a = Node::leaf('a', 1);
        let sum = compute_total_len(a.collect_info_of_char());
        assert_eq!(sum, 1);

        let a = Node::leaf('a', 1);
        let b = Node::leaf('b', 1);
        let ab = Node::tree(a, b);
    }
}

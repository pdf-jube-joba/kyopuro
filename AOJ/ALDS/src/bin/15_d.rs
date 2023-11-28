// if char is none, it represents end of string
#[derive(Debug, Clone, PartialEq)]
struct InfoChar {
    c: Option<char>,
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
    fn end_char() -> Self {
        Node::Leaf(Leaf::end_char())
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
}

use std::cmp::{Ord, Ordering, PartialOrd};
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Node::Leaf(l1), Node::Leaf(l2)) => l1.cmp(&l2),
            (Node::Tree(t1), Node::Tree(t2)) => t1.cmp(&t2),
            // if frequency is equal, leaf is smaller then tree
            (Node::Leaf(l), Node::Tree(t)) => l.frequency.cmp(&t.frequency).then(Ordering::Less),
            (Node::Tree(t), Node::Leaf(l)) => t.frequency.cmp(&l.frequency).then(Ordering::Greater),
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
        let frequency = left.frequency() + right.frequency();
        Self {
            frequency,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    fn collect_info_of_char(&self) -> Vec<InfoChar> {
        let mut v = vec![];

        for mut info in self.left.as_ref().collect_info_of_char() {
            info.depth += 1;
            v.push(info);
        }

        for mut info in self.right.as_ref().collect_info_of_char() {
            info.depth += 1;
            v.push(info);
        }
        v
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

// if char is none, it represents end of string and frequency of it is 0
#[derive(Debug, Clone, PartialEq, Eq)]
struct Leaf {
    c: Option<char>,
    frequency: usize,
}

impl Leaf {
    fn end_char() -> Self {
        Self {
            c: None,
            frequency: 0,
        }
    }
    fn new(c: char, frequency: usize) -> Self {
        Self {
            c: Some(c),
            frequency,
        }
    }
    fn collect_info_of_char(&self) -> InfoChar {
        InfoChar {
            c: self.c,
            frequency: self.frequency,
            depth: 0,
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

fn construct_huffman_tree_from_nodes(nodes: impl IntoIterator<Item = Node>) -> Node {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};
    let mut freq_priority_queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    for node in nodes {
        freq_priority_queue.push(Reverse(node));
    }

    debug_assert!(freq_priority_queue.len() > 1);

    while freq_priority_queue.len() > 1 {
        let node1 = freq_priority_queue.pop().unwrap().0;
        let node2 = freq_priority_queue.pop().unwrap().0;
        let node_tree = Node::tree(node1, node2);
        freq_priority_queue.push(Reverse(node_tree));
    }
    freq_priority_queue
        .pop()
        .map(|rev_node| rev_node.0)
        .unwrap()
}

fn construct_huffman_tree(str: &[char]) -> Node {
    use std::collections::HashMap;
    let mut count = HashMap::new();
    for c in str {
        count.entry(*c).and_modify(|count| *count += 1).or_insert(1);
    }

    // <=> str is empty
    if count.is_empty() {
        return Node::end_char();
    }

    let mut nodes: Vec<Node> = vec![];

    // if there is only one kind of char
    // we insert sentinel to make the Huffman tree complete tree
    // this is ugly because theory is ugly
    if count.len() == 1 {
        nodes.push(Node::end_char());
    }

    // min heap of node by frequency
    for (c, frequency) in count {
        let node = Node::leaf(c, frequency);
        nodes.push(node);
    }

    construct_huffman_tree_from_nodes(nodes)
}

fn compute_total_len(v: Vec<InfoChar>) -> usize {
    v.into_iter().map(|info| info.frequency * info.depth).sum()
}

fn main() {
    let str = input();
    let tree = construct_huffman_tree(&str);
    let infos = tree.collect_info_of_char();
    let sum = compute_total_len(infos);
    println!("{}", sum);
}

fn input() -> Vec<char> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    buf.trim().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_test() {
        let a = Node::tree(Node::leaf('a', 1), Node::end_char());
        let infos = a.collect_info_of_char();
        let sum = compute_total_len(infos);
        assert_eq!(sum, 1);

        let a = Node::tree(
            Node::leaf('a', 3),
            Node::tree(Node::leaf('b', 1), Node::end_char()),
        );
        let infos = a.collect_info_of_char();

        let sum = compute_total_len(infos);
        assert_eq!(sum, 5);
    }
    #[test]
    fn huffman_nodes_test() {
        let nodes = vec![
            Node::leaf('a', 45),
            Node::leaf('c', 12),
            Node::leaf('b', 13),
            Node::leaf('d', 16),
            Node::leaf('f', 5),
            Node::leaf('e', 9),
        ];
        let tree = construct_huffman_tree_from_nodes(nodes);
        println!("{:?}", tree);
    }
    #[test]
    fn huffman_test() {
        fn test(str: &str, sum: usize) {
            let tree = construct_huffman_tree(&(str.chars().collect::<Vec<_>>()));
            let infos = tree.collect_info_of_char();
            assert_eq!(compute_total_len(infos), sum);
        }
        // empty case
        test("", 0);

        // one char
        test("a", 1);
        test("aa", 2);
        test("aaa", 3);
        test("aaaaaa", 6);

        // two char
        test("ab", 2);
        test("aaab", 4);
        test("bbabba", 6);

        // three char
        test("abc", 5);
        test("aabc", 6);
        test("aaabc", 7);
        test("aabbc", 8);
        test("aabbcc", 10);
        test("aabbbcc", 11);

        // four char
        test("abcd", 8);
        test("aabcd", 10);
        test("aaabcd", 11);
        test("aabbcd", 12);
        test("aabbccd", 14);
        test("aabbccdd", 16);
    }
}

use std::collections::vec_deque::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    left_id: Option<usize>,
    right_id: Option<usize>,
}

fn get_parent_id(v: &[Node]) -> usize {
    let mut is_p = vec![true; v.len()];
    for node in v {
        if let Some(left_id) = node.left_id {
            is_p[left_id] = false;
        }
        if let Some(right_id) = node.right_id {
            is_p[right_id] = false;
        }
    }
    is_p.into_iter().enumerate().find_map(
        |(i, b)| if b { Some(i) } else { None }
    ).unwrap()
}

#[derive(Debug, Clone)]
enum Move {
    Left,
    Right,
}



struct PreIter<'a> {
    nodes: &'a[Node],
    parent_id: usize,
    pred_id: Vec<Move>,
}

fn preorder_iter<'a>(nodes: &'a [Node]) -> PreIter<'a> {
    let parent_id = get_parent_id(nodes);
    PreIter { nodes, parent_id, pred_id: vec![] }
}

impl<'a> Iterator for PreIter<'a> {
    type Item = usize;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> { 
        if self.pred_id.is_empty() {
            None
        } else {
            let now = {
                let id = self.parent_id;
                for mov in &self.pred_id {
                    match mov {
                        Move::Left => self.nodes[id].left_id,
                        Move::Right => self.nodes[id].right_id,
                    }
                }
                id
            };


            Some(now)
        }
    }
}

fn get_nodes(v: Vec<(usize, Option<usize>, Option<usize>)>) -> Vec<Node> {
    let mut nodes = vec![Node { left_id: None, right_id: None}; v.len()];
    for (id, left_id, right_id) in v {
        nodes[id].left_id = left_id;
        nodes[id].right_id = right_id
    }
    nodes
}

fn main() {
    let nodes = get_nodes(input());
}

use std::convert::TryFrom;
fn input() -> Vec<(usize, Option<usize>, Option<usize>)> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..n).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let v: Vec<isize> = buf.split_whitespace().map(|str| str.parse::<isize>().unwrap()).collect();
        (
            usize::try_from(v[0]).unwrap(),
            if v[1] < 0 { None } else { Some(usize::try_from(v[1]).unwrap()) },
            if v[2] < 0 { None } else { Some(usize::try_from(v[2]).unwrap()) }
        )
    }).collect()
}
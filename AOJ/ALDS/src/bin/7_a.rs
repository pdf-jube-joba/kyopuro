#[derive(Debug, Clone)]
struct Arena {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    key: usize,
    depth: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Arena {
    fn new(children_ids: Vec<Vec<usize>>) -> Option<Self> {
        let n = children_ids.len();
        if n == 0 {
            return Some(Arena {
                nodes: vec![]
            });
        }
        let parent_id: usize = {
            let mut is_children = vec![false; n];
            for i in 0..n {
                children_ids[i].iter().for_each(|i|{
                    is_children[*i] = true;
                })
            }
            is_children.into_iter().enumerate().find_map(|(i, b)| if !b { Some(i) } else { None }).unwrap()
        };

        let mut nodes: Vec<Option<Node>> = vec![None; n];

        let mut stack = vec![(parent_id, None, 0)];

        while let Some((key, parent, depth)) = stack.pop() {
            nodes[key] = Some( Node {
                key,
                depth,
                parent,
                children: children_ids[key].clone(),
            });
            for cid in &children_ids[key] {
                stack.push((*cid, Some(key), depth + 1));
            }
        }

        Some(Arena { nodes: nodes.into_iter().collect::<Option<_>>()? })

    }
    fn get_index(&self, pointer: usize) -> Option<&Node> {
        self.nodes.get(pointer)
    }
    fn get_mut_index(&mut self, pointer: usize) -> Option<&mut Node> {
        self.nodes.get_mut(pointer)
    }
    fn is_root(&self, pointer: usize) -> Option<bool> {
        self.get_index(pointer).map(|node| node.parent.is_none())
    }
    fn is_leaf(&self, pointer: usize) -> Option<bool> {
        self.get_index(pointer).map(|node| node.children.is_empty())
    }
    fn is_internal_node(&self, pointer: usize) -> Option<bool> {
        self.is_leaf(pointer).map(|b| !b)
    }
    fn parent_of(&self, pointer: usize) -> Option<usize> {
        self.get_index(pointer).and_then(|node| node.parent)
    }
    fn children_of(&self, pointer: usize) -> Option<&[usize]> {
        self.get_index(pointer).map(|node| node.children.as_ref())
    }
    fn depth_of(&self, pointer: usize) -> Option<usize> {
        self.get_index(pointer).map(|node| node.depth)
    }
}

fn main() {
    let tree = input();
    let max_n = tree.nodes.len() - 1;
    for i in 0..=max_n {
        println!(
            "node {}: parent = {}, depth = {}, {}, [{}]",
            i,
            match tree.parent_of(i) {
                Some(i) => i.to_string(),
                None => "-1".to_string(),
            },
            tree.depth_of(i).unwrap(),
            {
                if tree.is_root(i).unwrap() {
                    "root".to_string()
                } else if tree.is_leaf(i).unwrap() {
                    "leaf".to_string()
                } else if tree.is_internal_node(i).unwrap() {
                    "internal node".to_string()
                } else {
                    unreachable!()
                }
            },
            tree.children_of(i).unwrap().iter().map(|cid| cid.to_string()).collect::<Vec<_>>().join(", "),
        )
    }
}

fn input() -> Arena {
    use std::io::Read;
    let mut string = String::new();
    std::io::stdin().read_to_string(&mut string).unwrap();
    let mut lines = string.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let mut v: Vec<(usize, usize, Vec<usize>)> = lines.map(|str|{
        let mut num_iter = 
            str.split_whitespace().map(|str| str.parse::<usize>().unwrap());
        (num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.collect())
    }).collect();
    v.sort_by(|node_info1, node_info2| node_info1.0.cmp(&node_info2.0));
    let v = v.into_iter().map(|(_, _, v)| v).collect();
    Arena::new(v).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arena() {
        let ids = vec![
        ];
        let _ = Arena::new(ids).unwrap();

        let ids = vec![
            vec![],
        ];
        let _ = Arena::new(ids).unwrap();

        let ids = vec![
            vec![1],
            vec![],
        ];
        let _ = Arena::new(ids).unwrap();

        let ids = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![],
        ];
        let _ = Arena::new(ids).unwrap();
        let ids = vec![
            vec![1,2,3],
            vec![4],
            vec![5],
            vec![6],
            vec![],
            vec![],
            vec![],
        ];
        let _ = Arena::new(ids).unwrap();

        let ids = vec![
            vec![],
            vec![0],
        ];
        let _ = Arena::new(ids).unwrap();

    }
}
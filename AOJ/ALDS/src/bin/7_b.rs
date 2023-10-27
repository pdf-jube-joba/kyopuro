use std::collections::vec_deque::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    left_id: Option<usize>,
    right_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Root,
    Internal,
    Leaf,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Kind::Root => "root".to_owned(),
            Kind::Internal => "internal node".to_owned(),
            Kind::Leaf => "leaf".to_owned(),
        }        
    }
}

#[derive(Debug, Clone)]
struct NodeInfo {
    parent_id: Option<usize>,
    sibling: Option<usize>,
    child_num: usize,
    depth: usize,
    height: usize,
    kind: Kind,
}

#[derive(Debug, Default, Clone)]
struct NodeInfoBuild {
    parent_id: Option<usize>,
    sibling: Option<usize>,
    child_num: Option<usize>,
    depth: Option<usize>,
    height: Option<usize>,
    kind: Option<Kind>,
}

fn safe_build(nodeinfobuilder: NodeInfoBuild) -> Option<NodeInfo> {
    Some(NodeInfo {
        parent_id: nodeinfobuilder.parent_id,
        sibling: nodeinfobuilder.sibling,
        child_num: nodeinfobuilder.child_num?,
        depth: nodeinfobuilder.depth?,
        height: nodeinfobuilder.height?,
        kind: nodeinfobuilder.kind?,
    })
}

fn format(id: usize, info: &NodeInfo) -> String {
    format!(
        "node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}",
        id,
        if let Some(u) = info.parent_id { u.to_string() } else {"-1".to_owned()},
        if let Some(u) = info.sibling { u.to_string() } else {"-1".to_owned()},
        info.child_num,
        info.depth,
        info.height,
        info.kind.to_string(),
    )
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

fn construct(v: Vec<Node>) -> Vec<NodeInfo> {
    let mut builder: Vec<_> = vec![NodeInfoBuild::default(); v.len()];

    let mut queue: VecDeque<_> =
        VecDeque::from(vec![(get_parent_id(&v), None, None, 0)]);
    let mut leaf_memo: VecDeque<_> = VecDeque::new();
    
    while let Some((id, parent_id, sibling, depth)) = queue.pop_front() {
        let kind = if parent_id.is_none() {
            Kind::Root
        } else if v[id].left_id.is_none() && v[id].right_id.is_none() {
            Kind::Leaf
        } else {
            Kind::Internal
        };

        builder[id].kind = Some(kind);
        builder[id].parent_id = parent_id;
        builder[id].depth = Some(depth);
        builder[id].sibling = sibling;
        builder[id].child_num = match (v[id].left_id, v[id].right_id) {
            (Some(_), Some(_)) => Some(2),
            (Some(_), None) | (None, Some(_)) => Some(1),
            _ => Some(0),
        };

        if v[id].left_id.is_none() {
            leaf_memo.push_back((id, 0));
        }

        if let Some(left_id) = v[id].left_id {
            queue.push_back((left_id, Some(id), v[id].right_id, depth + 1));
        }
        if let Some(right_id) = v[id].right_id {
            queue.push_back((right_id, Some(id), v[id].left_id, depth + 1));
        }
    }

    while let Some((id, height)) = leaf_memo.pop_front() {
        builder[id].height = match builder[id].height {
            None => Some(height),
            Some(height2) => Some(std::cmp::max(height, height2)),
        };
        if let Some(parent_id) = builder[id].parent_id {
            leaf_memo.push_back((parent_id, height + 1));
        }
    }

    builder.into_iter().map(|m| {
        safe_build(m).unwrap()
    }).collect()
}

fn main() {
    let nodes = get_nodes(input());
    let node_infos = construct(nodes);
    for (id, info) in node_infos.into_iter().enumerate() {
        println!("{}", format(id, &info))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_test() {
        // one node case
        let nodes = get_nodes(vec![(0, None, None)]);
        let _ = construct(nodes);

        // id is reverse
        let nodes = get_nodes(vec![
            (0, None, None),
            (1, Some(0), None),
            (2, Some(1), None),
            (3, Some(2), None),
        ]);
        let _ = construct(nodes);
    }
}
// #[derive(Debug, Clone)]
// struct Node {
//     left_id: Option<usize>,
//     right_id: Option<usize>,
// }

#[derive(Debug, Clone, PartialEq)]
struct TreeRec {
    id: usize,
    left: Option<Box<TreeRec>>,
    right: Option<Box<TreeRec>>,
}

impl TreeRec {
    fn new(id: usize, left: Option<TreeRec>, right: Option<TreeRec>) -> TreeRec {
        Self {
            id,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}

// fn flatten(treerec: TreeRec) -> Vec<Node> {
//     let nodes = vec![

//     ];
// }

fn post_order_vec(tree: &TreeRec) -> Vec<usize> {
    let mut v = vec![];
    if let Some(ref left_tree) = tree.left {
        v.extend(post_order_vec(left_tree));
    }
    if let Some(ref right_tree) = tree.right {
        v.extend(post_order_vec(right_tree));
    }
    v.push(tree.id);
    v
}

fn construct_tree_rec(preorder: &[usize], inorder: &[usize]) -> Option<TreeRec> {
    debug_assert!(preorder.len() == inorder.len());

    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let this_id = preorder[0];
    let id_inorder_index = inorder
        .iter()
        .enumerate()
        .find_map(|(index, id)| if this_id == *id { Some(index) } else { None })
        .unwrap();

    let preorder_left = &preorder[1..=id_inorder_index];
    let inorder_left = &inorder[0..id_inorder_index];
    let left_tree = construct_tree_rec(preorder_left, inorder_left);

    let preorder_right = &preorder[id_inorder_index + 1..];
    let inorder_right = &inorder[id_inorder_index + 1..];
    let right_tree = construct_tree_rec(preorder_right, inorder_right);

    Some(TreeRec {
        id: this_id,
        left: left_tree.map(|tree| Box::new(tree)),
        right: right_tree.map(|tree| Box::new(tree)),
    })
}

fn main() {
    let (fst, snd) = input();
    let tree = construct_tree_rec(&fst, &snd).unwrap();
    let post_order_vec = post_order_vec(&tree);
    println!("{}", {
        post_order_vec.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
    })
}

fn input() -> (Vec<usize>, Vec<usize>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap();
    };
    let fst = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    let snd = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    (fst, snd)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        let preorder = vec![];
        let inorder = vec![];
        let tree = construct_tree_rec(&preorder, &inorder);
        assert_eq!(tree, None);

        let preorder = vec![1];
        let inorder = vec![1];
        let tree = construct_tree_rec(&preorder, &inorder);
        assert_eq!(
            tree,
            Some(TreeRec::new(
                1,
                None,
                None,
            ))
        );

        let preorder = vec![1, 2, 3];
        let inorder = vec![2, 1, 3];
        let tree = construct_tree_rec(&preorder, &inorder);
        assert_eq!(
            tree,
            Some(TreeRec::new(
                1,
                Some(TreeRec::new(2, None, None)),
                Some(TreeRec::new(3, None, None)),
            ))
        );

        let preorder = vec![1, 2, 3];
        let inorder = vec![1, 2, 3];
        let tree = construct_tree_rec(&preorder, &inorder);
        assert_eq!(
            tree,
            Some(TreeRec::new(
                1,
                None,
                Some(TreeRec::new(
                    2,
                    None,
                    Some(TreeRec::new(
                        3, None, None,
                    ))
                ))
            ))
        );

        let preorder = vec![0, 2, 3];
        let inorder = vec![2, 0, 3];
        let tree = construct_tree_rec(&preorder, &inorder);
        assert_eq!(
            tree,
            Some(TreeRec::new(
                0,
                Some(TreeRec::new(2, None, None)),
                Some(TreeRec::new(3, None, None)),
            ))
        );
    }
}

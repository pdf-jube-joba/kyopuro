use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct TreeNode {
    key: isize,
    parent: Option<Weak<RefCell<TreeNode>>>,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(key: isize) -> TreeNode {
        TreeNode {
            key,
            parent: None,
            left: None,
            right: None,
        }
    }
}

struct Tree {
    tree: Option<Rc<RefCell<TreeNode>>>,
}

fn left_borrow(r: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    r.borrow().left.as_ref().map(Rc::clone)
}

fn right_borrow(r: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    r.borrow().right.as_ref().map(Rc::clone)
}

fn insert(tree: &mut Tree, mut z: TreeNode) {
    let mut y: Option<Rc<RefCell<TreeNode>>> = None;
    let mut x: Option<Rc<RefCell<TreeNode>>> = (&tree.tree).as_ref().map(|t| Rc::clone(&t));
    while let Some(x_treenode) = x {
        y = Some(Rc::clone(&x_treenode));
        if z.key < x_treenode.borrow().key {
            x = left_borrow(Rc::clone(&x_treenode));
        } else {
            x = right_borrow(Rc::clone(&x_treenode));
        }
    }

    z.parent = y.as_ref().map(|r| Rc::downgrade(&r));

    match y {
        None => {
            tree.tree = Some(Rc::new(RefCell::new(z)));
        }
        Some(y) => {
            if z.key < y.borrow().key {
                y.borrow_mut().left = Some(Rc::new(RefCell::new(z)));
            } else {
                y.borrow_mut().right = Some(Rc::new(RefCell::new(z)));
            }
        }
    }
}

fn find_rec(tree_node: &Rc<RefCell<TreeNode>>, key: isize) -> bool {
    let this_key = tree_node.borrow().key;
    match this_key.cmp(&key) {
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => {
            if let Some(left_tree_ref) = &tree_node.borrow().left {
                find_rec(left_tree_ref, key)
            } else {
                false
            }
        }
        std::cmp::Ordering::Less => {
            if let Some(right_tree_ref) = &tree_node.borrow().right {
                find_rec(right_tree_ref, key)
            } else {
                false
            }
        }
    }
}

fn find(tree: &Tree, key: isize) -> bool {
    match &tree.tree {
        None => false,
        Some(tree_node) => find_rec(tree_node, key),
    }
}

fn preorder_vec(tree: &Tree) -> Vec<isize> {
    fn preorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<isize> {
        let mut v = vec![];
        v.push(tree_node.borrow().key);
        if let Some(left_treenode) = &tree_node.borrow().left {
            v.extend(preorder_vec_rec(left_treenode));
        }
        if let Some(right_treenode) = &tree_node.borrow().right {
            v.extend(preorder_vec_rec(right_treenode));
        }
        v
    }
    if let Some(r) = &tree.tree {
        preorder_vec_rec(r)
    } else {
        vec![]
    }
}

fn inorder_vec(tree: &Tree) -> Vec<isize> {
    fn inorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<isize> {
        let mut v = vec![];
        if let Some(left_treenode) = &tree_node.borrow().left {
            v.extend(inorder_vec_rec(left_treenode));
        }
        v.push(tree_node.borrow().key);
        if let Some(right_treenode) = &tree_node.borrow().right {
            v.extend(inorder_vec_rec(right_treenode));
        }
        v
    }
    if let Some(r) = &tree.tree {
        inorder_vec_rec(r)
    } else {
        vec![]
    }
}

fn postorder_vec(tree: &Tree) -> Vec<isize> {
    fn postorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<isize> {
        let mut v = vec![];
        if let Some(left_treenode) = &tree_node.borrow().left {
            v.extend(postorder_vec_rec(left_treenode));
        }
        if let Some(right_treenode) = &tree_node.borrow().right {
            v.extend(postorder_vec_rec(right_treenode));
        }
        v.push(tree_node.borrow().key);
        v
    }
    if let Some(r) = &tree.tree {
        postorder_vec_rec(r)
    } else {
        vec![]
    }
}

fn main() {
    let order = input();
    let mut tree = Tree { tree: None };
    for order in order {
        match order {
            Order::Insert(key) => {
                insert(&mut tree, TreeNode::new(key));
            }
            Order::Print => {
                println!(" {}", {
                    inorder_vec(&tree)
                        .into_iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                });
                println!(" {}", {
                    preorder_vec(&tree)
                        .into_iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                });
            }
            Order::Find(key) => println!("{}", {
                if find(&tree, key) {
                    "yes"
                } else {
                    "no"
                }
            }),
        }
    }
}

enum Order {
    Insert(isize),
    Find(isize),
    Print,
}

fn input() -> Vec<Order> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            if &buf[0..6] == "insert" {
                Order::Insert((&buf[6..]).trim().parse::<isize>().unwrap())
            } else if &buf[0..4] == "find" {
                Order::Find((&buf[5..]).trim().parse::<isize>().unwrap())
            } else {
                debug_assert!(buf == "print\n");
                Order::Print
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        let mut tree = Tree { tree: None };
        assert_eq!(preorder_vec(&tree), vec![]);
        assert_eq!(inorder_vec(&tree), vec![]);
        assert_eq!(postorder_vec(&tree), vec![]);

        insert(&mut tree, TreeNode::new(0));
        assert_eq!(preorder_vec(&tree), vec![0]);
        assert_eq!(inorder_vec(&tree), vec![0]);
        assert_eq!(postorder_vec(&tree), vec![0]);

        insert(&mut tree, TreeNode::new(1));
        assert_eq!(preorder_vec(&tree), vec![0, 1]);
        assert_eq!(inorder_vec(&tree), vec![0, 1]);
        assert_eq!(postorder_vec(&tree), vec![1, 0]);

        insert(&mut tree, TreeNode::new(2));
        assert_eq!(preorder_vec(&tree), vec![0, 1, 2]);
        assert_eq!(inorder_vec(&tree), vec![0, 1, 2]);
        assert_eq!(postorder_vec(&tree), vec![2, 1, 0]);
    }
}

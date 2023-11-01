use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::cmp::Ordering;


#[derive(Debug)]
struct TreeNode {
    key: usize,
    priority: usize,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(key: usize, priority: usize) -> TreeNode {
        TreeNode {
            key,
            priority,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct Tree {
    tree: Option<Rc<RefCell<TreeNode>>>,
}

fn left_borrow(r: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    r.borrow().left.as_ref().map(Rc::clone)
}

fn right_borrow(r: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    r.borrow().right.as_ref().map(Rc::clone)
}

fn right_rotate(y_node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let maybe_left = y_node.borrow_mut().left.take();
    let x_node: Rc<RefCell<TreeNode>> = if let Some(left) = maybe_left {
        left
    } else {
        return y_node;
    };

    let b: Option<Rc<RefCell<TreeNode>>> = x_node.borrow_mut().right.take();
    
    y_node.borrow_mut().left = b;
    x_node.borrow_mut().right = Some(y_node);

    x_node
}

fn left_rotate(x_node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let maybe_right = x_node.borrow_mut().right.take();
    let y_node: Rc<RefCell<TreeNode>> = if let Some(right) = maybe_right {
        right
    } else {
        return x_node;
    };

    let b: Option<Rc<RefCell<TreeNode>>> = y_node.borrow_mut().left.take();
    
    x_node.borrow_mut().right = b;
    y_node.borrow_mut().left = Some(x_node);

    y_node
}

fn insert_key_priority_rec(tree_node: Option<Rc<RefCell<TreeNode>>>, key: usize, priority: usize) -> Rc<RefCell<TreeNode>> {
    let mut tree_node: Rc<RefCell<_>> = if let Some(tree_node) = tree_node {
        tree_node
    } else {
        return Rc::new(RefCell::new(TreeNode::new(key, priority)))
    };
    let ord = {
        key.cmp(&tree_node.borrow().key)
    };
    match ord {
        Ordering::Equal => {
        }
        Ordering::Less => {
            let left = tree_node.borrow_mut().left.take();
            let new_left = insert_key_priority_rec(left, key, priority);
            let new_left_priority = new_left.borrow().priority;
            tree_node.borrow_mut().left = Some(new_left);
            if tree_node.borrow().priority < new_left_priority {
                tree_node = right_rotate(tree_node);
            }
        }
        Ordering::Greater => {
            let right = tree_node.borrow_mut().right.take();
            let new_right = insert_key_priority_rec(right, key, priority);
            let new_right_priority = new_right.borrow().priority;
            tree_node.borrow_mut().right = Some(new_right);
            if tree_node.borrow().priority < new_right_priority {
                tree_node = left_rotate(tree_node);
            }
        }
    }
    tree_node
}

fn insert_key_priority(tree: &mut Tree, key: usize, priority: usize) {
    let root_node = tree.tree.take();
    let new_root_node = insert_key_priority_rec(root_node, key, priority);
    tree.tree = Some(new_root_node);
}

fn find_rec(tree_node: &Rc<RefCell<TreeNode>>, key: usize) -> bool {
    let this_key = tree_node.borrow().key;
    match this_key.cmp(&key) {
        Ordering::Equal => true,
        Ordering::Greater => {
            if let Some(left_tree_ref) = &tree_node.borrow().left {
                find_rec(left_tree_ref, key)
            } else {
                false
            }
        }
        Ordering::Less => {
            if let Some(right_tree_ref) = &tree_node.borrow().right {
                find_rec(right_tree_ref, key)
            } else {
                false
            }
        }
    }
}

fn find(tree: &Tree, key: usize) -> bool {
    match &tree.tree {
        None => false,
        Some(tree_node) => find_rec(tree_node, key),
    }
}

fn find_rec_tree(tree_node: &Rc<RefCell<TreeNode>>, key: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let this_key = tree_node.borrow().key;
    match this_key.cmp(&key) {
        Ordering::Equal => {
            Some(Rc::clone(tree_node))
        },
        Ordering::Greater => {
            if let Some(left_tree_ref) = &tree_node.borrow().left {
                find_rec_tree(left_tree_ref, key)
            } else {
                None
            }
        }
        Ordering::Less => {
            if let Some(right_tree_ref) = &tree_node.borrow().right {
                find_rec_tree(right_tree_ref, key)
            } else {
                None
            }
        }
    }
}

enum DelRes {
    WellDel,
    NotFound,
    DeleteThis(Option<Rc<RefCell<TreeNode>>>),
}

fn delete_rec(tree_node: Option<Rc<RefCell<TreeNode>>>, key: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let tree_node = tree_node?;
    let ord = {
        key.cmp(&tree_node.borrow().key)
    };
    match ord {
        Ordering::Equal => {
            return delete_this_node(tree_node, key);
        }
        Ordering::Less => {
            let left = tree_node.borrow_mut().left.take();
            let new_left = delete_rec(left, key);
            tree_node.borrow_mut().left = new_left;
        }
        Ordering::Greater => {
            let right = tree_node.borrow_mut().right.take();
            let new_right = delete_rec(right, key);
            tree_node.borrow_mut().right = new_right;
        }
    }
    Some(tree_node)
}

fn delete_this_node(mut tree_node: Rc<RefCell<TreeNode>>, key: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let left = tree_node.borrow_mut().left.take();
    let right = tree_node.borrow_mut().right.take();
    match (left, right) {
        (None, None) => {
            return None;
        },
        (None, Some(right)) => {
            tree_node.borrow_mut().right = Some(right);
            tree_node = left_rotate(tree_node);
        }
        (Some(left), None) => {
            tree_node.borrow_mut().left = Some(left);
            tree_node = right_rotate(tree_node);
        }
        (Some(left), Some(right)) => {
            let left_priority = left.borrow().priority;
            let right_priority = right.borrow().priority;
            tree_node.borrow_mut().left = Some(left);
            tree_node.borrow_mut().right = Some(right);
            if left_priority > right_priority {
                tree_node = right_rotate(tree_node);
            } else {
                tree_node = left_rotate(tree_node);
            }
        }
    }
    delete_rec(Some(tree_node), key)
}

fn delete(tree: &mut Tree, key: usize) {
    let root_node = tree.tree.take();
    tree.tree = delete_rec(root_node, key)
}

fn preorder_vec(tree: &Tree) -> Vec<usize> {
    fn preorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<usize> {
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

fn inorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<usize> {
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

fn inorder_vec(tree: &Tree) -> Vec<usize> {
    if let Some(r) = &tree.tree {
        inorder_vec_rec(r)
    } else {
        vec![]
    }
}

fn postorder_vec(tree: &Tree) -> Vec<usize> {
    fn postorder_vec_rec(tree_node: &Rc<RefCell<TreeNode>>) -> Vec<usize> {
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
            Order::Insert(key, priority) => {
                insert_key_priority(&mut tree, key, priority);
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
            Order::Delete(key) => {
                delete(&mut tree, key);
            }
        }
    }
}

enum Order {
    Insert(usize, usize),
    Find(usize),
    Delete(usize),
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
                let v = (&buf[6..]).split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>();
                Order::Insert(v[0], v[1])
            } else if &buf[0..4] == "find" {
                Order::Find((&buf[5..]).trim().parse::<usize>().unwrap())
            } else if &buf[0..6] == "delete" {
                Order::Delete((&buf[6..]).trim().parse::<usize>().unwrap())
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
        let mut tree = Tree {
            tree: None,
        };
        insert_key_priority(&mut tree, 0, 0);

        assert_eq!(inorder_vec(&tree), vec![0]);
        assert_eq!(preorder_vec(&tree), vec![0]);

        insert_key_priority(&mut tree, 1, 1);
        assert_eq!(inorder_vec(&tree), vec![0,1]);
        assert_eq!(preorder_vec(&tree), vec![1,0]);

        insert_key_priority(&mut tree, 2, 2);
        assert_eq!(inorder_vec(&tree), vec![0,1,2]);
        assert_eq!(preorder_vec(&tree), vec![2,1,0]);

        delete(&mut tree, 0);

    }
}

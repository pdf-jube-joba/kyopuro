use std::io::Read;
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
struct Tree {
    id: usize,
    depth: RefCell<usize>,
    parent: Weak<RefCell<Tree>>,
    children: Vec<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn id(&self) -> usize {
        self.id
    }
    fn depth(&self) -> usize {
        *self.depth.borrow()
    }
    fn parent(&self) -> Option<Rc<RefCell<Tree>>> {
        self.parent.upgrade()
    }
    fn children(&self) -> &[Rc<RefCell<Tree>>] {
        &self.children
    }
    fn is_root(&self) -> bool {
        self.parent().is_none()
    }
    fn is_internal_node(&self) -> bool {
        !self.children().is_empty()
    }
    fn is_leaf(&self) -> bool {
        self.children().is_empty()
    }
}

fn from_child(
    child_ids: Vec<Vec<usize>>
) -> Option<Rc<RefCell<Tree>>> {
    let n = child_ids.len();
    let parent_id = {
        let mut v = vec![true; n];
        for cids in &child_ids {
            for cid in cids {
                v[*cid] = false;
            }
        }
        v.into_iter().enumerate().find_map(|(i, b)|
            if b { Some(i) } else { None }
        )?
    };

    let mut this_tree = new(parent_id);

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&this_tree));
    while let Some(mut tree) = queue.pop_front() {
        // let mut rcref_tree = search_from_id(&this_tree, next_id)?;
        let copied: &[usize] = &child_ids[tree.borrow().id];
        for &cid in copied {
            let t = new(cid);
            queue.push_back(Rc::clone(&t));
            insert_this(&mut tree, t);
        }
    }
    normalize(&mut this_tree);

    Some(this_tree)
}

fn max_id(tree: &Rc<RefCell<Tree>>) -> usize {
    let c_max = tree.borrow().children.iter().map(|rc|{
        max_id(rc)
    }).max();
    if let Some(max) = c_max {
        std::cmp::max(max, tree.borrow().id())
    } else {
        tree.borrow().id()
    }
}

fn new(
    id: usize
) -> Rc<RefCell<Tree>> {
    Rc::new(RefCell::new(Tree { 
        id,
        depth: RefCell::new(0),
        parent: Weak::new(),
        children: vec![]
    }))
}

fn new_rcref_tree(
    id: usize,
    depth: usize,
    children: Vec<Rc<RefCell<Tree>>>,
) -> Rc<RefCell<Tree>> {
    let mut this_tree = Rc::new(RefCell::new(Tree { 
        id,
        depth: RefCell::new(depth),
        parent: Weak::new(),
        children,
    }));
    normalize(&mut this_tree);
    this_tree
}

fn search_from_id(tree: &Rc<RefCell<Tree>>, id: usize) -> Option<Rc<RefCell<Tree>>> {
    if tree.borrow().id == id {
        Some(Rc::clone(tree))
    } else {
        tree.borrow().children.iter().find_map(|refcell_rc_tree|{
            search_from_id(refcell_rc_tree, id)
        })
    }
}

fn insert_this(tree: &mut Rc<RefCell<Tree>>, child: Rc<RefCell<Tree>>) {
    tree.borrow_mut().children.push(child);
}

fn insert_part(tree: &mut Rc<RefCell<Tree>>, id: usize, child: Rc<RefCell<Tree>>) {
    let ref_to_tree = search_from_id(&tree, id);
    if let Some(mut rc_tree) = ref_to_tree {
        rc_tree.borrow_mut().children.push(child);
    }
}

fn depth_calc(tree: &mut Rc<RefCell<Tree>>) {    
    let mut depth_stack: Vec<(Rc<RefCell<Tree>>, usize)> = vec![(Rc::clone(&tree), 0)];
    while let Some((tree, depth)) = depth_stack.pop() {
        *tree.borrow_mut().depth.borrow_mut() = depth;
        for c in &tree.borrow().children {
            depth_stack.push((Rc::clone(c), depth + 1));
        }
    }
}

fn parent_calc(tree: &mut Rc<RefCell<Tree>>) {
    let mut parent_stack: Vec<(Rc<RefCell<Tree>>, Weak<_>)> = vec![(Rc::clone(&tree), Weak::new())];
    while let Some((tree, parent)) = parent_stack.pop() {
        tree.borrow_mut().parent = parent;
        for c in &tree.borrow().children {
            parent_stack.push((Rc::clone(c), Rc::downgrade(&tree)));
        }
    }
}

fn normalize(tree: &mut Rc<RefCell<Tree>>) {
    parent_calc(tree);
    depth_calc(tree);
}

fn main() {
    let children_ids = input_str(&input(std::io::stdin()));
    if children_ids.len() >= 100 {
        println!("hello");
    }
    let tree = from_child(children_ids).unwrap();
    let max_id = max_id(&tree);
    for id in 0..=max_id {
        let tree = search_from_id(&tree, id).unwrap();
        println!(
            "node {}: parent = {}, depth = {}, {}, [{}]",
            id,
            if let Some(parent) = tree.borrow().parent() {
                parent.borrow().id().to_string()
            } else {
                "-1".to_owned()
            },
            tree.borrow().depth(),
            if tree.borrow().is_root() {
                "root"
            } else if tree.borrow().is_internal_node() {
                "internal node"
            } else {
                assert!(tree.borrow().is_leaf());
                "leaf"
            },
            tree.borrow().children().iter().map(|child| child.borrow().id().to_string()).collect::<Vec<String>>().join(", ")
        );
    }
}

fn input<T: std::io::Read>(mut input: T) -> String {
    let mut string = String::new();
    input.read_to_string(&mut string).unwrap();
    string
}

fn input_str(str: &str) -> Vec<Vec<usize>> {
    let mut lines = str.lines();
    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut v = vec![vec![]; n];
    for line in lines {
        let mut words = line.split_whitespace();
        let id: usize = words.next().unwrap().parse::<usize>().unwrap();
        let _k: usize = words.next().unwrap().parse::<usize>().unwrap();
        let c: Vec<usize> = words.map(|word| word.parse::<usize>().unwrap()).collect();
        v[id] = c;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        let mut t1 = new(0);
        assert!(search_from_id(&t1, 0).is_some());

        insert_part(&mut t1, 0, new(1));
        insert_part(&mut t1, 0, new(2));
        insert_part(&mut t1, 1, new(3));
    }
    #[test]
    fn tree_depth_test() {
        let child_ids = vec![
            vec![1],
            vec![],
        ];
        let t = from_child(child_ids).unwrap();
        eprintln!("{t:?}");

        let child_ids = vec![
            vec![],
            vec![0],
        ];
        let t = from_child(child_ids).unwrap();
        eprintln!("{t:?}");
    }
    #[test]
    fn child_test() {
        let mut child_ids = vec![
            (1..100_000).collect(),
        ];
        child_ids.extend(vec![vec![]; 99_999]);
        let t = from_child(child_ids).unwrap();
    }
}
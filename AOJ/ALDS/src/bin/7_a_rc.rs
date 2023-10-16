use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Tree {
    id: usize,
    depth: usize,
    parent: Option<Rc<RefCell<Tree>>>,
    children: Vec<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn new_rc(
        id: usize,
        children: Vec<Tree>,
    ) -> Rc<RefCell<Tree>> {
        let this_tree = Rc::new(RefCell::new(Tree {
            id,
            parent: None,
            depth: 0,
            children: vec![],
        }));
        let children = children.into_iter().map(|mut tree| {
            tree.parent = Some(this_tree.clone());
            Rc::new(RefCell::new(tree))
        }).collect();
        this_tree.borrow_mut().children = children;
        this_tree
    }
}

fn main() {

}
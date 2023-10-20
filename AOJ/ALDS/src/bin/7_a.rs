use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
struct Tree {
    id: usize,
    depth: usize,
    parent: Weak<RefCell<Tree>>,
    children: Vec<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn id(&self) -> usize {
        self.id
    }
    fn depth(&self) -> usize {
        self.depth
    }
    fn parent(&self) -> Option<&Tree> {
        self.parent
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        // let t1 = Tree::new(0, vec![]);
        // let t2 = Tree::new(1, vec![]);
        // let t3 = Tree::new(2, vec![t1, t2]);
        // eprintln!("{:?}", t3);
        // let mut c = t3.children_get();
        // c = vec![];
        // eprintln!("{:?}", t3);
    }
}
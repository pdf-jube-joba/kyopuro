use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Tree {
    id: usize,
    depth: usize,
    parent: Option<ToTree>,
    children: Vec<ToTree>,
}

#[derive(Debug, Clone)]
struct ToTree(Rc<RefCell<Tree>>);

impl From<Rc<RefCell<Tree>>> for ToTree {
    fn from(value: Rc<RefCell<Tree>>) -> Self {
        ToTree(value)
    }
}

impl From<Tree> for ToTree {
    fn from(value: Tree) -> Self {
        ToTree(Rc::new(RefCell::new(value)))
    }
}

impl ToTree {
    fn id(self) -> usize {
        self.0.borrow().id
    }
    fn depth(self) -> usize {
        self.0.borrow().depth
    }
    fn parent(self) -> Option<ToTree> {
        self.0.borrow().parent.clone()
    }
    // fn parent_get(&self) -> &Option<ToTree> {
    //     self.0.borrow().parent.clone()
    // }
    fn children(self) -> Vec<ToTree> {
        self.0.borrow().children.clone()        
    }
    fn children_get(&self) -> Vec<ToTree> {
        self.0.borrow().children.clone()
    }
    fn new(
        id: usize,
        children: Vec<ToTree>,
    ) -> ToTree {
        let this_tree: ToTree = Tree {
            id,
            parent: None,
            depth: 0,
            children: children.into_iter().map(|tree| {
                tree.into()
            }).collect(),
        }.into();
        // for children_tree in this_tree.0.borrow().children {
        //     children_tree.0.borrow_mut().parent = Some(this_tree.clone());
        // }
        this_tree.into()
    }
    fn appered_id(&self) -> Vec<usize> {
        let mut v = vec![self.0.borrow().id];
        v.extend(self.0.borrow().children.iter().flat_map(|ref_tree| {
            ref_tree.appered_id()
        }));
        v
    }
    fn get_ref_from_id(&self, id: usize) -> Option<&ToTree> {
        if self.0.borrow().id == id {
            Some(self)
        } else {
            None
        }
    }
}

fn get_ref_from_id(tree: ToTree) {}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        let t1 = ToTree::new(0, vec![]);
        let t2 = ToTree::new(1, vec![]);
        let t3 = ToTree::new(2, vec![t1, t2]);
        eprintln!("{:?}", t3);
        let mut c = t3.children_get();
        c = vec![];
        eprintln!("{:?}", t3);
    }
}
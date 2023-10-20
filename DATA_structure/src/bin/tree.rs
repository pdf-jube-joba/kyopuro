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
}

fn new_rcref_tree(
    id: usize,
    depth: usize,
    children: Vec<Rc<RefCell<Tree>>>,
) -> Rc<RefCell<Tree>> {
    Rc::new(RefCell::new(Tree { 
        id,
        depth: RefCell::new(depth),
        parent: Weak::new(),
        children
    }))
}

fn search_from_id(tree: Rc<RefCell<Tree>>, id: usize) -> Option<Rc<RefCell<Tree>>> {
    if tree.borrow().id == id {
        Some(tree.clone())
    } else {
        tree.borrow().children.iter().find_map(|refcell_rc_tree|{
            search_from_id(refcell_rc_tree.clone(), id)
        })
    }
}

fn insert(mut tree: Rc<RefCell<Tree>>, id: usize, child: Rc<RefCell<Tree>>) {
    let mut ref_to_tree = search_from_id(tree, id);
    if let Some(mut rc_tree) = ref_to_tree {
        rc_tree.borrow_mut().children.push(child);
    }
}

fn depth_calc(mut tree: Rc<RefCell<Tree>>) {    
    let mut depth_stack: Vec<(Rc<RefCell<Tree>>, usize)> = vec![(Rc::clone(&tree), 0)];
    while let Some((tree, depth)) = depth_stack.pop() {
        *tree.borrow_mut().depth.borrow_mut() = depth;
        for c in &tree.borrow().children {
            depth_stack.push((Rc::clone(c), depth + 1));
        }
    }
}

fn parent_calc(mut tree: Rc<RefCell<Tree>>) {
    let mut parent_stack: Vec<(Rc<RefCell<Tree>>, Weak<_>)> = vec![(Rc::clone(&tree), Weak::new())];
    while let Some((tree, parent)) = parent_stack.pop() {
        tree.borrow_mut().parent = parent;
        for c in &tree.borrow().children {
            parent_stack.push((Rc::clone(c), Rc::downgrade(&tree)));
        }
    }
}

fn main() {
    let children_ids = input();
}

fn input() -> Vec<Vec<usize>> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.parse::<usize>().unwrap();
    
    let mut v: Vec<(usize, Vec<usize>)> = (0..n).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut words = buf.split_whitespace();
        let id: usize = words.next().unwrap().parse::<usize>().unwrap();
        let _k: usize = words.next().unwrap().parse::<usize>().unwrap();
        let c: Vec<usize> = words.into_iter().map(|word| word.parse::<usize>().unwrap()).collect();
        (id, c)
    }).collect();
    v.sort_by(|v1, v2| v1.0.cmp(&v2.0));
    v.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_test() {
        let t1 = new_rcref_tree(0, 0, vec![]);
        assert!(search_from_id(Rc::clone(&t1), 0).is_some());

        let t2 = new_rcref_tree(1, 0, vec![]);
        insert(Rc::clone(&t1), 0, t2);
        eprintln!("{t1:?}");
    }
}
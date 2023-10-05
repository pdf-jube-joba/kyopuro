#[derive(Debug, Clone)]
struct Tree {
    id: usize,
    children: Vec<Box<Tree>>,
}

impl Tree {
    fn appered_id(&self) -> Vec<usize> {
        let mut v = vec![self.id];
        v.extend(self.children.iter().flat_map(|boxed_tree| boxed_tree.as_ref().appered_id()));
        v
    }
    fn is_appered(&self, id: usize) -> bool {
        self.id == id || {
            self.children.iter().any(|boxed_tree|
                boxed_tree.as_ref().is_appered(id)
            )
        } 
    }
    fn is_root(&self, id: usize) -> bool {
        self.id == id
    }
    // return some(b) if it contains id as children otherwise return none
    fn is_leaf(&self, id: usize) -> Option<bool> {
        if self.id == id {
            Some(self.children.is_empty())
        } else {
            self.children.iter().find_map(|boxed_tree| 
                boxed_tree.as_ref().is_leaf(id)
            )
        }
    }
    fn is_internal_node(&self, id: usize) -> Option<bool> {
        if self.id == id {
            Some(!self.children.is_empty())
        } else {
            self.children.iter().find_map(|boxed_tree|
                boxed_tree.as_ref().is_internal_node(id)
            )
        }
    }
    fn parent_of(&self, id: usize) -> Option<usize> {
        if self.children.is_empty() {
            return None;
        }
        if self.children.iter().any(|boxed_tree|
            boxed_tree.as_ref().id == id
        ) {
            Some(self.id)
        } else {
            self.children.iter().find_map(|boxed_tree|
                boxed_tree.parent_of(id)
            )
        }
    }
    fn insert(&mut self, id: usize, tree: &Tree) -> Option<()> {
        if !self.is_appered(id) {
            return None;
        }
        if self.id == id {
            self.children.push(Box::new(tree.clone()));
            Some(())
        } else {
            self.children.iter_mut().find_map(|boxed_mut_tree|
                boxed_mut_tree.as_mut().insert(id, tree)
            )
        }
    }
    fn children_ids_of(&self, id: usize) -> Option<Vec<usize>> {
        if self.id == id {
            Some(self.children.iter().map(
                |boxed_tree| boxed_tree.as_ref().id
            ).collect::<Vec<usize>>())
        } else if self.children.is_empty() {
            None
        } else {
            self.children.iter().find_map(
                |boxed_tree| boxed_tree.as_ref().children_ids_of(id)
            )
        }
    }
    fn depth_of_rec(&self, id: usize, rec: usize) -> Option<usize> {
        if self.id == id {
            Some(rec)
        } else {
            self.children.iter().find_map(
                |boxed_tree| boxed_tree.as_ref().depth_of_rec(id, rec + 1)
            )
        }
    }
    fn depth_of(&self, id: usize) -> Option<usize> {
        self.depth_of_rec(id, 0)
    }
}

fn construct_from_child_ids_rec(children_ids: &[Vec<usize>], id: usize) -> Option<Tree> {
    if children_ids.len() <= id {
        return None;
    }
    let children = children_ids[id].iter().map(|cid|{
        construct_from_child_ids_rec(children_ids, *cid).map(
            |tree| Box::new(tree)
        )
    }).collect::<Option<_>>()?;
    Some(Tree {
        id,
        children, 
    })
}

fn construct_from_child_ids(children_ids: Vec<Vec<usize>>) -> Option<Tree> {
    let n = children_ids.len();
    if n == 0 {
        return None;
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

    let mut trees: Vec<Option<Tree>> = vec![None; n];
    let mut stack = vec![parent_id];

    while let Some(next_id) = stack.pop() {
        // leaf case
        if children_ids[next_id].is_empty() {
            trees[next_id] = Some(Tree {
                id: next_id,
                children: vec![],
            });
        } else {
            let unconstructed_children_id = children_ids[next_id].iter().filter(|cid| trees[**cid].is_none()).collect::<Vec<_>>();
            if unconstructed_children_id.is_empty() {
                let children_trees: Vec<Box<Tree>> = children_ids[next_id].iter().map(
                    |cid| Box::new(trees[*cid].take().unwrap())
                ).collect();

                trees[next_id] = Some(Tree {
                    id: next_id,
                    children: children_trees,
                })
            } else {
                stack.push(next_id);
                stack.extend(unconstructed_children_id);
            }
        }
    }
    trees.into_iter().find_map(|tree| tree)
}

fn main() {
    let tree = input();
    let ids = tree.appered_id();
    let max_n = *ids.iter().max().unwrap();
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
                if tree.is_root(i) {
                    "root".to_string()
                } else if tree.is_leaf(i).unwrap() {
                    "leaf".to_string()
                } else if tree.is_internal_node(i).unwrap() {
                    "internal node".to_string()
                } else {
                    unreachable!()
                }
            },
            tree.children_ids_of(i).unwrap().into_iter().map(|cid| cid.to_string()).collect::<Vec<_>>().join(", "),
        )
    }
}

use std::io::Read;
fn input() -> Tree {
    let mut string = String::new();
    std::io::stdin().read_to_string(&mut string).unwrap();
    input_from_str(&string)
}

fn input_from_str(str: &str) -> Tree {
    let mut lines = str.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let mut v: Vec<Vec<usize>> = lines.map(|str|{
        str.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
    }).collect();
    v.sort_by(|node_info1, node_info2| node_info1[0].cmp(&node_info2[0]));
    let v = v.into_iter().map(|v1| v1[2..].to_owned()).collect();

    construct_from_child_ids(v).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_construct_test() {
        let v: Vec<Vec<usize>> = vec![
            vec![],
        ];
        let t = construct_from_child_ids(v).unwrap();

        let v: Vec<Vec<usize>> = vec![
            vec![1],
            vec![],
        ];
        let t = construct_from_child_ids(v).unwrap();

        let v: Vec<Vec<usize>> = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![],
        ];
        let t = construct_from_child_ids(v).unwrap();
        eprintln!("{:?}", t);

        let v: Vec<Vec<usize>> = vec![
            vec![1,2,3],
            vec![4],
            vec![5],
            vec![6],
            vec![],
            vec![],
            vec![],
        ];
        let t = construct_from_child_ids(v).unwrap();
        eprintln!("{:?}", t);

        let v: Vec<Vec<usize>> = vec![
            vec![],
            vec![0],
        ];
        let t = construct_from_child_ids(v).unwrap();
        eprintln!("{:?}", t);
    }

    #[test]
    fn tree_construct_test_big() {
        let n = 100_000;
        let mut v: Vec<Vec<usize>> = Vec::with_capacity(n);
        v.push((1..n).collect());
        v.extend((1..n).map(|_| vec![]));

        let tree = construct_from_child_ids(v).unwrap();
    }
    #[test]
    fn input_tree_test() {
        let str: String = {
            let mut string = String::new();
            string.push_str("100000\n");
            string.push_str(&format!("0 999999 {}\n", {
                (1..100_000).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
            }));
            for i in 0..100_000 {
                string.push_str(&format!("{} 0\n", i));
            }
            string
        };
        let tree = input_from_str(&str);
        let n = *tree.appered_id().iter().max().unwrap();
        for i in 0..=n {
            let _ = tree.parent_of(i);
        }
    }
}
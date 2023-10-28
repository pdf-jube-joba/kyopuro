use std::collections::vec_deque::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    left_id: Option<usize>,
    right_id: Option<usize>,
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

#[derive(Debug, Clone, PartialEq)]
enum Move {
    Left,
    Right,
}

fn get_node_with_move(nodes: &[Node], moves: &[Move]) -> Option<usize> {
    let mut id = get_parent_id(nodes);
    for mov in moves {
        match mov {
            Move::Left => {
                id = nodes[id].left_id?;
            }
            Move::Right => {
                id = nodes[id].right_id?;
            }
        }
    }
    Some(id)
}

// compare X = [X1,...,Xn], Y = [Y1,...,Yn] <=> let i be min of Xi != Yi then
// X < Y <=> (Xi, Yi) = (None, Some(L)), (None, Some(R)), (Some(L), Some(R))
// (... Y < X <=> (Xi, Yi) = othercase)
// then iter is iterating over the valid moves ordered by this order
struct PreIter<'a> {
    nodes: &'a[Node],
    pred_move: Option<Vec<Move>>,
}

fn preorder_iter(nodes: &[Node]) -> PreIter {
    PreIter { nodes, pred_move: Some(vec![]) }
}

// next of move = X1,...,Xn is
// if X1,...,Xn,L is valid => X1,...,Xn,L
// else
//  - find i such that Xi = L and for any k > i, Xk = R
//  - if such i exist =>
//    - let new moves be X1,...,Xi-1,R
//    - if it is valid then go on
//    - if it is invalid then go to "find i such that"
//  - else <=> X1 = X2 = ... = Xn = R so next move is None
impl<'a> Iterator for PreIter<'a> {
    type Item = usize;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let moves = self.pred_move.as_ref()?;
        let now = get_node_with_move(&self.nodes, moves)?;

        let mut empty_flag = false;

        let mut next_moves = moves.clone();
        next_moves.push(Move::Left);

        while get_node_with_move(&self.nodes, &next_moves).is_none() {
            let i = next_moves.iter().enumerate().rev().find_map(|(i, m)| if *m == Move::Left { Some(i) } else { None });
            if let Some(i) = i {
                next_moves = next_moves[0..i].to_vec();
                next_moves.push(Move::Right);
            } else {
                empty_flag = true;
                break;
            }
        }
        
        if empty_flag {
            self.pred_move = None;
        } else {
            self.pred_move = Some(next_moves);
        }
        
        Some(now)
    }
}
// [],[L],[LL],[LR],[R],[RL],[RLL],[RLR],[RR]

// compare X = [X1,...,Xn], Y = [Y1,...,Yn] <=> let i be min of Xi != Yi then
// X < Y <=> (Xi, Yi) = (Some(L), None), (Some(L), Some(R)), (None, Some(R))
// (... Y < X <=> (Xi, Yi) = othercase)
// then iter is iterating over the valid moves ordered by this order
struct InIter<'a> {
    nodes: &'a[Node],
    pred_move: Option<Vec<Move>>,
}

fn inorder_iter(nodes: &[Node]) -> InIter {
    let mut v = vec![];
    while get_node_with_move(nodes, &v).is_some() {
        v.push(Move::Left);
    }
    let _ = v.pop();
    InIter { nodes, pred_move: Some(v) }
}

// next of move X1,...,Xn
// - if X1,...,Xn,R is valid, X1,...,Xn,R,L,...,L such that is valid and max num of L
// - else find i such that Xi = L and for any k > i, Xk = R
//   - if exists, X1,...,Xi-1
//   - else <=> X1 = ... = Xn = R so next is none
impl<'a> Iterator for InIter<'a> {
    type Item = usize;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let moves = self.pred_move.as_ref()?;
        let now = get_node_with_move(&self.nodes, moves)?;

        let mut empty_flag = false;

        let mut next_moves = moves.clone();
        next_moves.push(Move::Right);
        if get_node_with_move(&self.nodes, &next_moves).is_some() {
            while get_node_with_move(&self.nodes, &next_moves).is_some() {
                next_moves.push(Move::Left);
            }
            let _ = next_moves.pop();
        } else {
            let i = next_moves.iter().enumerate().rev().find_map(|(i, m)| if *m == Move::Left { Some(i) } else { None });
            if let Some(i) = i {
                next_moves = next_moves[0..i].to_vec();
            } else {
                empty_flag = true;
            }
        }

        if empty_flag {
            self.pred_move = None;
        } else {
            self.pred_move = Some(next_moves);
        }

        Some(now)
    }
}

// [LL],[L],[LR],[],[RLL],[RL],[RLR],[R],[RR]

// compare X = [X1,...,Xn], Y = [Y1,...,Yn] <=> let i be min of Xi != Yi then
// X < Y <=> (Xi, Yi) = (Some(L), Some(R)), (Some(L), None), (Some(R), None)
// (... Y < X <=> (Xi, Yi) = othercase)
// then iter is iterating over the valid moves ordered by this order
struct PostIter<'a> {
    nodes: &'a [Node],
    pred_move: Option<Vec<Move>>,
}

fn postorder_iter(nodes: &[Node]) -> PostIter {
    let mut v = vec![];
    while get_node_with_move(nodes, &v).is_some() {
        v.push(Move::Left);
        if get_node_with_move(nodes, &v).is_none() {
            v.pop();
            v.push(Move::Right);
        }
    }
    let _ = v.pop();
    PostIter { nodes, pred_move: Some(v) }
}

// next of move X1,...,Xn
// - if n != 0 
//   - if Xn = L
//     - if X1,...,Xn-1,R is valid => X1,...,Xn-1,R,Y1,...,Yn such that minimal and valid
//     - else X1,...,Xn-1 
//   - if Xn = R then X1,...,Xn-1
// - else <=> [X1,...,Xn] = [] so now move point root so next is none
impl<'a> Iterator for PostIter<'a> {
    type Item = usize;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let moves = self.pred_move.as_ref()?;
        let now = get_node_with_move(&self.nodes, moves)?;

        let mut next_moves = moves.clone();

        if next_moves.is_empty() {
            self.pred_move = None;
        } else {

            match next_moves[next_moves.len() - 1] {
                Move::Left => {
                    let _ = next_moves.pop();
                    next_moves.push(Move::Right);

                    if get_node_with_move(&self.nodes, &next_moves).is_none() {
                        let _ = next_moves.pop();
                    } else {
                        while get_node_with_move(&self.nodes, &next_moves).is_some() {
                            next_moves.push(Move::Left);
                            if get_node_with_move(&self.nodes, &next_moves).is_none() {
                                next_moves.pop();
                                next_moves.push(Move::Right);
                            }
                        }
                        let _ = next_moves.pop();
                    }
                }
                Move::Right => {
                    let _ = next_moves.pop();
                }
            }

            self.pred_move = Some(next_moves);

        }

        Some(now)
    }
}

// [LL],[LR],[L],[RLL],[RLR],[RL],[RR],[R],[]

fn get_nodes(v: Vec<(usize, Option<usize>, Option<usize>)>) -> Vec<Node> {
    let mut nodes = vec![Node { left_id: None, right_id: None}; v.len()];
    for (id, left_id, right_id) in v {
        nodes[id].left_id = left_id;
        nodes[id].right_id = right_id
    }
    nodes
}

fn main() {
    let nodes = get_nodes(input());
    let parent_id = get_parent_id(&nodes);
    println!("Preorder");
    println!(" {}", {
        preorder_iter(&nodes).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
    });
    println!("Inorder");
    println!(" {}", {
        inorder_iter(&nodes).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
    });
    println!("Postorder");
    println!(" {}", {
        postorder_iter(&nodes).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
    });
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
    fn get_node_with_move_test() {
        let nodes = get_nodes(vec![
            (0, Some(1), Some(2)),
            (1, None, None),
            (2, None, None),
        ]);
        let id = get_node_with_move(&nodes, &vec![]);
        assert_eq!(id, Some(0));
        let id = get_node_with_move(&nodes, &vec![Move::Left]);
        assert_eq!(id, Some(1));
        let id = get_node_with_move(&nodes, &vec![Move::Right]);
        assert_eq!(id, Some(2));
        let id = get_node_with_move(&nodes, &vec![Move::Left, Move::Left]);
        assert_eq!(id, None);
    }
    fn test_case_gen() -> Vec<Vec<Node>> {
        vec![
            get_nodes(vec![
                (0, None, None)
            ]),
            get_nodes(vec![
                (0, Some(1), Some(2)),
                (1, None, None),
                (2, None, None),
            ]),
            get_nodes(vec![
                (0, Some(1), None),
                (1, Some(2), None),
                (2, None, None),
            ]),
            get_nodes(vec![
                (0, Some(1), None),
                (1, None, Some(2)),
                (2, None, None),
            ]),
            get_nodes(vec![
                (0, None, Some(1)),
                (1, Some(2), None),
                (2, None, None),
            ]),
            get_nodes(vec![
                (0, None, Some(1)),
                (1, None, Some(2)),
                (2, None, None),
            ]),
            get_nodes(vec![
                (0, Some(1), Some(4)),
                (1, Some(2), Some(3)),
                (2, None, None),
                (3, None, None),
                (4, Some(5), Some(6)),
                (5, None, None),
                (6, None, None),
            ]),
        ]
    }
    #[test]
    fn pre_iter_test() {
        let expect = vec![
            vec![0],
            vec![0,1,2],
            vec![0,1,2],
            vec![0,1,2],
            vec![0,1,2],
            vec![0,1,2],
            vec![0,1,2,3,4,5,6],
        ];
        for (nodes, test) in test_case_gen().into_iter().zip(expect) {
            let res = preorder_iter(&nodes).collect::<Vec<_>>();
            assert_eq!(res, test)
        }
    }
    #[test]
    fn inorder_test() {
        let expect = vec![
            vec![0],
            vec![1,0,2],
            vec![2,1,0],
            vec![1,2,0],
            vec![0,2,1],
            vec![0,1,2],
            vec![2,1,3,0,5,4,6],
        ];
        for (nodes, test) in test_case_gen().into_iter().zip(expect) {
            let res = inorder_iter(&nodes).collect::<Vec<_>>();
            assert_eq!(res, test)
        }
    }
    #[test]
    fn postorder_test() {
        let expect = vec![
            vec![0],
            vec![1,2,0],
            vec![2,1,0],
            vec![2,1,0],
            vec![2,1,0],
            vec![2,1,0],
            vec![2,3,1,5,6,4,0],
        ];
        for (nodes, test) in test_case_gen().into_iter().zip(expect) {
            let res = postorder_iter(&nodes).collect::<Vec<_>>();
            assert_eq!(res, test)
        }
    }
}
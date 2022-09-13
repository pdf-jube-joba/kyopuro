fn main() {
    proconio::input! {
        n: usize, q:usize,
        ab: [(usize, usize); n-1],
        px: [(usize, usize); q],
    }

    let result = solve(n, &ab, &px);
    (0..n).for_each(|i|{
        print!("{} ", result[i]);
    });
}

#[derive(Debug)]
struct Tree {
    this_number: usize,
    nodes: Vec<Tree>,
}

impl Tree {
    fn new(n: usize, ab: &[(usize, usize)]) -> Tree {
        let mut c: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for &(a,b) in ab {
            c[a].push(b);
            c[b].push(a);
        }
        fn new_rec(n: usize, p: Option<usize>, c: &[Vec<usize>]) -> Tree {
            Tree {
                this_number: n,
                nodes: 
                    c[n].iter().filter(|&i| Some(*i) != p).map(|&i|{
                        new_rec(i, Some(n), c)
                    }).collect()
            }
        }
        new_rec(1, None, &c)
    }
}

fn sum_tree(tree: &Tree, px: &[(usize, usize)], dp: &mut [usize], sum: usize) {
    let dp_next: usize = px.iter().map(|p|{ if p.0 == tree.this_number {p.1} else {0}}).sum();
    let next = dp_next + sum;
    dp[tree.this_number - 1] = next;
    tree.nodes.iter().for_each(|node|{
        sum_tree(node, px, dp, next)
    });
}

fn solve(n: usize, ab: &[(usize, usize)], px: &[(usize, usize)]) -> Vec<usize> {
    let tree = Tree::new(n,&ab);
    let mut dp = vec![0;n];
    sum_tree(&tree, px, &mut dp, 0);
    dp
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_1(){
        let n = 4;
        let _q = 3;
        let ab = vec![(1,2),(2,3),(2,4)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result, vec![100,110,111,110]);
    }
    #[test]
    fn test_2(){
        let n = 3;
        let _q = 3;
        let ab = vec![(1,3),(2,3)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result, vec![100,111,101]);
    }
}
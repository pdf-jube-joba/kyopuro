#[derive(Debug, Clone)]
struct UnionTree {
    info: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    parent: usize,
    size: usize,
}

impl UnionTree {
    fn new(n: usize) -> Self {
        let nodes: Vec<Node> = (0..n).map(|i| Node{
            parent: i,
            size: 1,
        }).collect();
        Self {
            info: nodes
        }
    }
    fn root(&self, x: usize) -> usize {
        let info = &self.info[x];
        if info.parent == x {
            x
        } else {
            self.root(info.parent)
        }
    }
    fn is_eq(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn union(&mut self, x: usize, y: usize) {
        let (x, y) = (self.root(x), self.root(y));
        if x != y {
            if self.info[x].size < self.info[y].size {
                self.info[x].parent = y;
                self.info[y].size += self.info[x].size;
            } else {
                self.info[y].parent = x;
                self.info[x].size += self.info[y].size;
            }
        }
    }
}

fn main() {
    let (n, rel, ques) = input();
    let mut union_tree = UnionTree::new(n);
    for (x, y) in rel {
        union_tree.union(x, y);
    }
    for (x, y) in ques {
        println!("{}", {
            if union_tree.is_eq(x, y) {
                "yes"
            } else {
                "no"
            }
        });
    }
}

fn input() -> (usize, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    use std::io::BufRead;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let (n, m) = {
        stdin.read_line(&mut buf).unwrap();
        let v = buf
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };
    let rel = {
        (0..m)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();
                let v = buf
                    .split_whitespace()
                    .map(|str| str.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            })
            .collect()
    };
    let q = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    let ques = (0..q)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let v = buf
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1])
        })
        .collect();

    (n, rel, ques)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn union_test() {
        fn test_one_set(uni: &UnionTree, set: &[usize]) {
            for i in set {
                for j in set {
                    assert!(uni.is_eq(*i, *j));
                }
            }
        }

        fn test_diff_set(uni: &UnionTree, set1: &[usize], set2: &[usize]) {
            for i in set1 {
                for j in set2 {
                    assert!(!uni.is_eq(*i, *j));
                }
            }
        }

        let mut uni = UnionTree::new(5);
        for i in 0..5 {
            assert_eq!(uni.root(i), i);
            for j in 0..5 {
                assert!(if i == j {
                    uni.is_eq(i, j)
                } else {
                    !uni.is_eq(i, j)
                });
            }
        }
        uni.union(0, 1);
        let set1 = vec![0, 1];
        let set2 = vec![2];
        test_one_set(&uni, &set1);
        test_diff_set(&uni, &set1, &set2);

        uni.union(0, 1);
        assert!(uni.is_eq(0, 1));

        uni.union(1, 0);
        assert!(uni.is_eq(0, 1));

        test_one_set(&uni, &set1);
        test_diff_set(&uni, &set1, &set2);

        eprintln!("{:?}", uni);
        uni.union(0, 2);
        uni.union(3, 4);
        eprintln!("{:?}", uni);

        let set1 = vec![0,1,2];
        let set2 = vec![3,4];
        test_one_set(&uni, &set1);
        test_one_set(&uni, &set2);
        test_diff_set(&uni, &set1, &set2);

        uni.union(2, 4);
        eprintln!("{:?}", uni);
        test_one_set(&uni, &[0,1,2,3,4]);
        test_one_set(&uni, &[0,1,2,3,4]);
    }
    #[test]
    fn tle_test() {
        let n = 100_000;
        let m = 100_000;
        let rel = (0..m).map(|i| (i, (i+1) % m)).collect::<Vec<_>>();
        let mut uni = UnionTree::new(n);
        for (i,j) in rel {
            uni.union(i,j);
        }

        let ques = (0..10_000).map(|i| (i, (i + 100) % m)).collect::<Vec<_>>();
        for (x,y) in ques {
            uni.is_eq(x,y);
        }
    }
}

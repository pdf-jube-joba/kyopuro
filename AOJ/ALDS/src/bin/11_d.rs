struct UnionTree {
    vec: Vec<Option<usize>>,
}

impl UnionTree {
    fn new(n: usize) -> Self {
        Self { vec: vec![None; n] }
    }
    fn flat(&mut self) {
        for i in 0..self.vec.len() {
            let mut this = i;
            let mut change = vec![];
            while let Some(parent) = self.vec[this] {
                change.push(this);
                this = parent;
            }
            for v in change {
                self.vec[v] = Some(this);
            }
        }
    }
    fn find_root(&self, x: usize) -> usize {
        let mut this = x;
        while let Some(parent) = self.vec[this] {
            this = parent
        }
        this
    }
    fn is_eq(&self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
    fn union(&mut self, x: usize, y: usize) {
        let rootx = self.find_root(x);
        let rooty = self.find_root(y);
        if rootx != rooty {
            self.vec[rootx] = Some(rooty);
        }
    }
}

fn main() {
    let (n, rel, ques) = input();
    let mut union_tree = UnionTree::new(n);
    for (x, y) in rel {
        union_tree.union(x, y);
    }
    union_tree.flat();
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
    let mut buf = String::new();
    let stdin = std::io::stdin();
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
            assert_eq!(uni.find_root(i), i);
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

        uni.union(0, 2);
        uni.union(3, 4);
        let set1 = vec![0,1,2];
        let set2 = vec![3,4];
        test_one_set(&uni, &set1);
        test_one_set(&uni, &set2);
        test_diff_set(&uni, &set1, &set2);

        uni.flat();

        test_one_set(&uni, &set1);
        test_one_set(&uni, &set2);
        test_diff_set(&uni, &set1, &set2);

        uni.union(2, 4);
        test_one_set(&uni, &vec![0,1,2,3,4]);
        uni.flat();
        test_one_set(&uni, &vec![0,1,2,3,4]);
    }
}

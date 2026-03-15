fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    println!("{}", diff(&p, &q));
}

fn permutation<T: Clone>(first: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    struct Perm<T> {
        permutation: Option<Vec<usize>>,
        first: Vec<T>,
    }
    impl<T: Clone> Iterator for Perm<T> {
        type Item = Vec<T>;
        fn next(&mut self) -> Option<Self::Item> {
            let result = self.permutation.clone();
            let permutation = self.permutation.as_mut()?;
            let n = permutation.len();
            let max_index = (0..n - 1)
                .filter(|&i| permutation[i] < permutation[i + 1])
                .max();
            match max_index {
                None => {
                    self.permutation = None;
                }
                Some(max_index) => {
                    let swap_index = (max_index + 1..n)
                        .filter(|&i| permutation[i] > permutation[max_index])
                        .max()
                        .unwrap();
                    permutation.swap(max_index, swap_index);
                    permutation[max_index + 1..n].reverse();
                }
            };
            match result {
                None => None,
                Some(result) => {
                    let mut v = vec![];
                    for i in result {
                        v.push(self.first[i].clone());
                    }
                    Some(v)
                }
            }
        }
    }
    Perm {
        permutation: Some((0..first.len()).collect()),
        first,
    }
}

fn diff(p: &[usize], q: &[usize]) -> usize {
    let mut p_num = 0;
    let mut q_num = 0;
    for (i, perm) in permutation((1..=p.len()).collect()).enumerate() {
        if *p == perm {
            p_num = i;
        }
        if *q == perm {
            q_num = i;
        }
    }
    p_num.abs_diff(q_num)
}

#[cfg(test)]
mod test {
    use crate::diff;

    #[test]
    fn diff_test_1() {
        assert_eq!(diff(&[1, 3, 2], &[3, 1, 2]), 3);
    }
    #[test]
    fn diff_test_2() {
        let a = vec![7, 3, 5, 4, 2, 1, 6, 8];
        let b = vec![3, 8, 2, 5, 4, 6, 7, 1];
        assert_eq!(diff(&a, &b), 17517);
    }
}

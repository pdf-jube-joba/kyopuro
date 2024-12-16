fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    println!("{}", average(&xy));
}

fn permutation(n: usize) -> impl Iterator<Item = Vec<usize>> {
    struct Perm {
        permutation: Option<Vec<usize>>,
    }
    impl Iterator for Perm {
        type Item = Vec<usize>;
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
            result
        }
    }
    Perm {
        permutation: Some((0..n).collect()),
    }
}

fn average(xy: &[(usize, usize)]) -> f32 {
    let n = xy.len();
    let a: Vec<f32> = permutation(n)
        .map(|perm| {
            eprintln!("{perm:?}");
            (0..n - 1)
                .map(|i| {
                    let (x1, y1) = xy[perm[i]];
                    let (x2, y2) = xy[perm[i + 1]];
                    ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).sqrt()
                })
                .sum()
        })
        .collect();
    a.iter().sum::<f32>() / (a.len() as f32)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn permutation_test() {
        let mut a = permutation(3);
        assert_eq!(a.next(), Some(vec![0, 1, 2]));
        assert_eq!(a.next(), Some(vec![0, 2, 1]));
        assert_eq!(a.next(), Some(vec![1, 0, 2]));
        assert_eq!(a.next(), Some(vec![1, 2, 0]));
        assert_eq!(a.next(), Some(vec![2, 0, 1]));
        assert_eq!(a.next(), Some(vec![2, 1, 0]));
    }

    #[test]
    fn average_test_1() {
        let xy = vec![(0, 0), (0, 1), (1, 0)];
        assert!(average(&xy) - 2.276_142_4 < 0.01)
    }
}

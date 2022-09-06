fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    println!("{}", average(&xy));

}

fn permutation(n: usize) -> impl Iterator<Item = Vec<usize>> {
    struct Perm {
        n: usize,
        perm: Option<Vec<usize>>,
    }
    impl Iterator for Perm {
        type Item = Vec<usize>;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(a) = &mut self.perm {
                let n = self.n;
                let max_index = match (0..n-1).filter(|i|{a[*i] < a[i+1]}).max() {
                    Some(i) => i,
                    None => {return None;}
                };
                let swap_index = {
                    (max_index+1..n).filter(|i|{a[*i] > a[max_index]}).max().unwrap()
                };
                { // swap a[max_index] and a[swap_index]
                    let temp = a[max_index];
                    a[max_index] = a[swap_index];
                    a[swap_index] = temp;
                }
                a[max_index+1..n].reverse();
                Some(a.clone())
            } else {
                self.perm = Some((0..self.n).collect());
                self.perm.clone()
            }
        }
    }
    Perm {n: n, perm: None}
}

fn average(xy: &[(usize, usize)]) -> f32 {
    let average = |a: &[f32]| -> f32 {
        a.iter().sum::<f32>() / (a.len() as f32)
    };
    let n = xy.len();
    let a: Vec<f32> = permutation(n).map(|perm|{
        (0..n-1).map(|i|{
            let (x1, y1) = xy[perm[i]];
            let (x2, y2) = xy[perm[i+1]];
            ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).sqrt()
        }).sum()
    }).collect();
    average(&a)
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
        let xy = vec![(0,0), (0,1), (1,0)];
        assert!(average(&xy) - 2.2761423749 < 0.01)
    }
}
fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    println!("{}", average(&xy));

}

fn permutation(n: usize) -> impl Iterator<Item = Vec<usize>> {
    struct Perm {
        permutation: Vec<usize>,
    }
    impl Iterator for Perm {
        type Item = Vec<usize>;
        fn next(&mut self) -> Option<Self::Item> {
            let result = Some(self.permutation.clone());
            let n = self.permutation.len();
            let max_index = match (0..n-1).filter(|&i|{self.permutation[i] < self.permutation[i+1]}).max() {
                Some(i) => i,
                None => {return None;}
            };
            let swap_index = {
                (max_index+1..n).filter(|&i|{self.permutation[i] > self.permutation[max_index]}).max().unwrap()
            };
            { // swap a[max_index] and a[swap_index]
                let temp = self.permutation[max_index];
                self.permutation[max_index] = self.permutation[swap_index];
                self.permutation[swap_index] = temp;
            }
            self.permutation[max_index+1..n].reverse();
            return result;
        }
    }
    Perm {permutation: (0..n).collect()}
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
use std::iter;

use itertools::all;

fn main() {
    proconio::input! {
        n: usize,
        c: [usize; n],
        x: [usize; n],
    }
    println!("{}", cost_sort(c, x))
}

fn lsb(i: usize) -> usize {
    let i = i as isize;
    (i & -i) as usize
}

#[derive(Debug, Clone, PartialEq)]
struct Bit {
    v: Vec<usize>,
}

impl Bit {
    fn new() -> Self {
        Bit { v: vec![0] }
    }
    fn new_with_size(size: usize) -> Self {
        Bit {
            v: vec![0; size + 1],
        }
    }
    // push new elm to a
    fn push(&mut self, new: usize) {
        let n = self.v.len();
        let sum: usize = (0..)
            .map(|i| 1 << i)
            .take_while(|d| *d != lsb(n))
            .map(|d| self.v[n - d])
            .sum();
        self.v.push(sum + new);
    }
    // sum[r] = sum of a[i] where i in 0..=r
    fn sum(&self, index: usize) -> usize {
        iter::successors(Some(index), |&i| Some(i - lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.v[i])
            .sum()
    }
    // a[index] += amount
    fn add(&mut self, index: usize, amount: usize) {
        let n = self.v.len();
        let mut i = index + 1;
        while i < n {
            self.v[i] += amount;
            i += lsb(i);
        }
    }
}

fn cost_sort(c: Vec<usize>, x: Vec<usize>) -> usize {
    let n = c.len();
    // by_color[c] = [x[i_0], ..., x[i_{n_k - 1}]] where i_k enumerate all of i in 0..n s.t. c[i] == c
    let by_color = {
        let mut by_color = vec![vec![]; *c.iter().max().unwrap() + 1];
        for i in 0..n {
            by_color[c[i]].push(x[i]);
        }
        by_color
    };
    let all_inv: usize = inversion_number_count(&x);
    let sum_each_c_inv: usize = by_color
        .into_iter()
        .inspect(|c| eprintln!("col: {:?}", c))
        .map(|by_color_c| inversion_number_count(&by_color_c))
        .inspect(|inv| eprintln!("inv: {}", inv))
        .sum();
    eprintln!("all:{}", all_inv);
    all_inv - sum_each_c_inv
}

fn inversion_number_count(a: &Vec<usize>) -> usize {
    let Some(&max_elm) = a.iter().max() else {
        // a is empty so inversion number = 0
        return 0;
    };
    let mut bit = Bit::new_with_size(max_elm + 1);
    let mut ans = 0;
    for i in 0..a.len() {
        eprintln!("{bit:?} ord:{}", bit.sum(a[i]));
        ans += i - bit.sum(a[i]);
        bit.add(a[i], 1);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inv_t() {
        // assert_eq!(inversion_number_count(&vec![0, 1, 2]), 0);
        // assert_eq!(inversion_number_count(&vec![1, 0, 2]), 1);
        // assert_eq!(inversion_number_count(&vec![0, 2, 1]), 1);
        // assert_eq!(inversion_number_count(&vec![2, 0, 1]), 2);
        // assert_eq!(inversion_number_count(&vec![2, 1, 0]), 3);
        // assert_eq!(inversion_number_count(&vec![1, 2, 8]), 0);
        assert_eq!(inversion_number_count(&vec![3, 2, 1, 2]), 4)
    }
}

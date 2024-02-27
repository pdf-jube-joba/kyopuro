use std::{collections::HashMap, iter};

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
    fn new_with_size(size: usize) -> Self {
        Bit {
            v: vec![0; size + 1],
        }
    }
    // sum[r] = sum of a[i] where i in 0..r
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
        .filter(|ci| ci.len() >= 2)
        .map(|ci| compress(&ci)) // if len < 2 then inv of a is 0
        .map(|by_color_c| inversion_number_count(&by_color_c))
        .sum();
    all_inv - sum_each_c_inv
}

fn inversion_number_count(a: &Vec<usize>) -> usize {
    let Some(&max_elm) = a.iter().max() else {
        return 0;
    };
    let mut bit = Bit::new_with_size(max_elm + 1);
    let mut ans = 0;
    // n times
    for i in 0..a.len() {
        // log n
        ans += i - bit.sum(a[i] + 1);
        bit.add(a[i], 1);
    }
    ans
}

fn compress(a: &Vec<usize>) -> Vec<usize> {
    let mut a: Vec<(usize, usize)> = a.iter().cloned().enumerate().collect();
    a.sort_by_key(|&(i, ai)| ai);
    a.into_iter().map(|(i, _ai)| i).collect()
}

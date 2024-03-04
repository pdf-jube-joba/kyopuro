use std::collections::BinaryHeap;

use ac_library::ModInt998244353 as ModInt;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", num(a))
}

fn num(a: Vec<usize>) -> ModInt {
    let n = a.len();
    let compress = compress(&a);
    num_perm(compress)
}

// input: A[0..n] which is permutation of (0..n)
// output: sum_{0 <= i < j < n s.t. A[i] < A[j]} 2^{j - i - 1}
// == sum_{0 <= j < n} C(j)
//   C(j) := 2^(j - 1) * B(j)
//   B(j) := sum_{0 <= i < j s.t. A[i] < A[j]} 2^(-i)
fn num_perm(a: Vec<usize>) -> ModInt {
    let n = a.len();
    let mut bit = Bit::new_with_size(n);
    let mut inv = ModInt::new(1);
    let mut ans: ModInt = ModInt::new(0);
    for j in 0..n {
        // inv = 2^(-j), bit[t] = if (exists i in 0..j s.t. t = a[i] < a[j]) then 2^(-i) else 0
        // => sum of bit[0..A[j]] == B(j)
        let bj = bit.sum(a[j]);
        let s = bj * ModInt::new(2).pow(j as u64) / 2;
        ans += s;

        bit.add(a[j], inv);
        inv /= 2;
    }
    ans
}

fn compress(a: &Vec<usize>) -> Vec<usize> {
    let mut a: Vec<(usize, usize)> = a.iter().cloned().enumerate().collect();
    // stable sort なのでよい。
    a.sort_by_key(|&(i, ai)| ai);
    a.iter_mut().enumerate().for_each(|(i, j)| {
        j.1 = i;
    });
    a.sort_by_key(|(i, ai_new)| *i);
    a.into_iter().map(|(i, ai_new)| ai_new).collect()
}

fn lsb(i: usize) -> usize {
    let i = i as isize;
    (i & -i) as usize
}

#[derive(Debug, Clone, PartialEq)]
struct Bit {
    v: Vec<ModInt>,
}

impl Bit {
    fn new_with_size(size: usize) -> Self {
        Bit {
            v: vec![ModInt::new(0); size],
        }
    }
    // sum[r] = sum_{i in 0..r} a[i]
    fn sum(&self, index: usize) -> ModInt {
        std::iter::successors(Some(index), |&i| Some(i - lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.v[i])
            .sum()
    }
    // a[index] += amount
    fn add(&mut self, index: usize, amount: ModInt) {
        let n = self.v.len();
        let mut i = index + 1;
        while i < n {
            self.v[i] += amount;
            i += lsb(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use permutohedron::LexicalPermutation;

    use super::*;
    fn g(a: &Vec<usize>) -> ModInt {
        let n = a.len();
        let mut ans = ModInt::new(0);
        for j in 0..n {
            let g = (0..j)
                .filter(|&i| a[i] < a[j])
                .map(|i| ModInt::new(2).pow((j - i - 1) as u64))
                .sum::<ModInt>();
            ans += g;
        }
        ans
    }
    #[test]
    fn cp() {
        let mut a: Vec<usize> = (0..5).collect_vec();
        while a.next_permutation() {
            let na = a.iter().cloned().map(|i| 2 * i).collect_vec();
            assert_eq!(compress(&na), a);
        }
    }
    #[test]
    fn t() {
        let mut a: Vec<usize> = (0..5).collect_vec();
        while a.next_permutation() {
            let res1 = g(&a);
            let res2 = num_perm(a.clone());
            assert_eq!(res1, res2)
        }
    }
}

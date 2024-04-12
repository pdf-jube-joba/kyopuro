fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    println!("{}", merge_sets(a));
}

fn merge_sets(a: Vec<Vec<usize>>) -> usize {
    let (n, m) = (a.len(), a[0].len());
    let mut all = vec![];
    for mut ai in a {
        ai.sort();
        all.extend(ai);
    }
    all = compress(&all);
    (n * (n - 1) / 2) * (m * (m + 1) / 2) + inversion_number_count(&all)
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
        std::iter::successors(Some(index), |&i| Some(i - lsb(i)))
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

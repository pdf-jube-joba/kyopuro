use itertools::Itertools;
use permutohedron::LexicalPermutation;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
    }
    println!("{}", make_takahashi_happy(a))
}

fn make_takahashi_happy(a: Vec<Vec<usize>>) -> usize {
    let (h, w) = (a.len(), a[0].len());
    let ms = vec![false; w - 1]
        .into_iter()
        .chain(vec![true; h - 1])
        .collect_vec();
    std::iter::successors(Some(ms), |ms| {
        let mut ms = ms.clone();
        if ms.next_permutation() {
            Some(ms.to_vec())
        } else {
            None
        }
    })
    .filter(|mm| is_happy_move(&a, mm))
    .count()
}

// false => right, true => down
fn is_happy_move(a: &Vec<Vec<usize>>, mm: &Vec<bool>) -> bool {
    let mut now = (0, 0);
    let mut ans = vec![a[0][0]];
    for &m in mm {
        if m {
            now.0 += 1;
        } else {
            now.1 += 1;
        }
        ans.push(a[now.0][now.1]);
    }
    ans.sort();
    let l = ans.len();
    ans.dedup();
    l == ans.len()
}

use im_rc::HashSet;
use itertools::iproduct;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize, l: usize,
        a: [usize; n],
        b: [usize; m],
        cds: [(Usize1, Usize1); l],
    }
    println!("{}", max_cost(a, b, cds));
}

fn max_cost(a: Vec<usize>, b: Vec<usize>, cds: Vec<(usize, usize)>) -> usize {
    let (n, m, l) = (a.len(), b.len(), cds.len());
    let mut b = b.into_iter().enumerate().collect::<Vec<_>>();
    b.sort_by_key(|(i, bi)| *bi);
    // cannot_use[c] = { d | (c, d) \in cds }
    let cannot_use: Vec<HashSet<usize>> = {
        let mut v = vec![HashSet::new(); n];
        for (c, d) in cds {
            v[c].insert(d);
        }
        v
    };
    (0..n)
        .filter_map(|i| {
            (0..m).rev().find_map(|j| {
                if !cannot_use[i].contains(&b[j].0) {
                    Some(a[i] + b[j].1)
                } else {
                    None
                }
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_cost_test() {
        let a = vec![2, 1];
        let b = vec![10, 30, 20];
        let cds = vec![(0, 1), (1, 0), (1, 2)];
        assert_eq!(max_cost(a, b, cds), 31);
    }
}

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        p: [Usize1; n],
        i: [Usize1; n],
    }
    if p[0] != 0 {
        println!("-1");
    } else if let Some(tree) = pre_order_and_in_order(p, i) {
        println!(
            "{}",
            tree.into_iter()
                .map(|(l, r)| {
                    let l = l.map(|l| l + 1).unwrap_or_default();
                    let r = r.map(|r| r + 1).unwrap_or_default();
                    format!("{} {}", l, r)
                })
                .join("\n")
        )
    } else {
        println!("-1");
    }
}

fn pre_order_and_in_order(
    p: Vec<usize>,
    i: Vec<usize>,
) -> Option<Vec<(Option<usize>, Option<usize>)>> {
    let n = p.len();
    let find = {
        let mut i = i.iter().cloned().enumerate().collect_vec();
        i.sort_by_key(|(i, ii)| *ii);
        i.into_iter().map(|(i, ii)| i).collect_vec()
    };
    let mut nodes = vec![None; n];
    pre_order_and_in_order_rec(&(p, i, find), (0, n, 0, n), &mut nodes)?;
    let nodes = nodes.into_iter().map(|nodes| nodes.unwrap()).collect();
    Some(nodes)
}

fn pre_order_and_in_order_rec(
    refs: &(Vec<usize>, Vec<usize>, Vec<usize>),
    (pl, pr, il, ir): (usize, usize, usize, usize),
    nodes: &mut [Option<(Option<usize>, Option<usize>)>],
) -> Option<()> {
    debug_assert!(pr >= pl && ir >= il && pr - pl == ir - il);
    let len = pr - pl;
    if len == 0 {
        return Some(());
    }
    let (p, i, find) = refs;
    let r = p[pl];
    let is = *find.get(r).filter(|&ind| (il..ir).contains(ind))?;
    // p[pl] p[pl+1..ps+1] p[ps+1..pr]
    // i[il..is] i[is] i[is+1..ir]
    let ps = pl + is - il;
    pre_order_and_in_order_rec(refs, (pl + 1, ps + 1, il, is), nodes)?;
    pre_order_and_in_order_rec(refs, (ps + 1, pr, is + 1, ir), nodes)?;
    nodes[r] = Some((
        p[pl + 1..ps + 1].first().cloned(),
        p[ps + 1..pr].first().cloned(),
    ));
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        let p = vec![];
        let i = vec![];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some(vec![]));

        let p = vec![0];
        let i = vec![0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some(vec![(None, None)]));

        let p = vec![0, 1];
        let i = vec![0, 1];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some(vec![(None, Some(1)), (None, None)]));

        let p = vec![0, 1];
        let i = vec![1, 0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some(vec![(Some(1), None), (None, None)]));
    }
    #[test]
    fn t2() {
        let p = vec![0, 1, 2];
        let i = vec![0, 1, 2];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some(vec![(None, Some(1)), (None, Some(2)), (None, None)])
        );

        let p = vec![0, 1, 2];
        let i = vec![0, 2, 1];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some(vec![(None, Some(1)), (Some(2), None), (None, None)])
        );

        let p = vec![0, 1, 2];
        let i = vec![1, 0, 2];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some(vec![(Some(1), Some(2)), (None, None), (None, None)])
        );

        let p = vec![0, 1, 2];
        let i = vec![1, 2, 0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some(vec![(Some(1), None), (None, Some(2)), (None, None)])
        );

        let p = vec![0, 1, 2];
        let i = vec![2, 0, 1];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, None);

        let p = vec![0, 1, 2];
        let i = vec![2, 1, 0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some(vec![(Some(1), None), (Some(2), None), (None, None)])
        );
    }
    #[test]
    fn t3() {
        let n = 200_000;
        let res = pre_order_and_in_order((0..n).collect_vec(), (0..n).rev().collect_vec());
    }
}

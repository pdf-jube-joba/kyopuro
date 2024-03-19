use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        p: [Usize1; n],
        i: [Usize1; n],
    }
    println!(
        "{}",
        match pre_order_and_in_order(p, i) {
            Some((head, tree)) if head == Some(0) => {
                tree.into_iter()
                    .map(|(l, r)| {
                        let l = l.map(|l| l + 1).unwrap_or_default();
                        let r = r.map(|r| r + 1).unwrap_or_default();
                        format!("{} {}", l, r)
                    })
                    .join("\n")
            }
            _ => {
                "-1".to_string()
            }
        }
    )
}

fn pre_order_and_in_order(
    p: Vec<usize>,
    i: Vec<usize>,
) -> Option<(Option<usize>, Vec<(Option<usize>, Option<usize>)>)> {
    let n = p.len();
    // find_i[l] = k s.t. i[k] == l
    let find = {
        let mut i = i.iter().cloned().enumerate().collect_vec();
        i.sort_by_key(|(i, ii)| *ii);
        i.into_iter().map(|(i, ii)| i).collect_vec()
    };
    let mut nodes: Vec<Option<(Option<usize>, Option<usize>)>> = vec![None; n];
    let mut stack: Vec<(usize, usize, usize, usize)> = vec![(0, n, 0, n)];
    while let Some((pl, pr, il, ir)) = stack.pop() {
        debug_assert!(pr >= pl && ir >= il && pr - pl == ir - il);
        let len = pr - pl;
        if len == 0 {
            continue;
        }

        let r = p[pl];

        let is = *find.get(r).filter(|&ind| (il..ir).contains(ind))?;
        let ps = pl + is - il;

        let left_head = p[pl + 1..ps + 1].first().cloned();
        let right_head = p[ps + 1..pr].first().cloned();

        let is_left_ok = match left_head {
            None => true,
            Some(left_head) if nodes[left_head].is_some() => true,
            _ => false,
        };
        let is_right_ok = match right_head {
            None => true,
            Some(right_head) if nodes[right_head].is_some() => true,
            _ => false,
        };

        if is_left_ok && is_right_ok {
            nodes[r] = Some((left_head, right_head));
        } else {
            stack.push((pl, pr, il, ir));
            if !is_left_ok {
                stack.push((pl + 1, ps + 1, il, is));
            }
            if !is_right_ok {
                stack.push((ps + 1, pr, is + 1, ir));
            }
        }
    }

    let nodes = nodes.into_iter().map(|nodes| nodes.unwrap()).collect();
    Some((p.first().cloned(), nodes))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        let p = vec![];
        let i = vec![];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some((None, vec![])));

        let p = vec![0];
        let i = vec![0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some((Some(0), vec![(None, None)])));

        let p = vec![0, 1];
        let i = vec![0, 1];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some((Some(0), vec![(None, Some(1)), (None, None)])));

        let p = vec![0, 1];
        let i = vec![1, 0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(res, Some((Some(0), vec![(Some(1), None), (None, None)])));
    }
    #[test]
    fn t2() {
        let p = vec![0, 1, 2];
        let i = vec![0, 1, 2];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some((
                Some(0),
                vec![(None, Some(1)), (None, Some(2)), (None, None)]
            ))
        );

        let p = vec![0, 1, 2];
        let i = vec![0, 2, 1];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some((
                Some(0),
                vec![(None, Some(1)), (Some(2), None), (None, None)]
            ))
        );

        let p = vec![0, 1, 2];
        let i = vec![1, 0, 2];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some((
                Some(0),
                vec![(Some(1), Some(2)), (None, None), (None, None)]
            ))
        );

        let p = vec![0, 1, 2];
        let i = vec![1, 2, 0];
        let res = pre_order_and_in_order(p, i);
        assert_eq!(
            res,
            Some((
                Some(0),
                vec![(Some(1), None), (None, Some(2)), (None, None)]
            ))
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
            Some((
                Some(0),
                vec![(Some(1), None), (Some(2), None), (None, None)]
            ))
        );
    }
    #[test]
    fn t3() {
        let n = 200_000;
        let res = pre_order_and_in_order((0..n).collect_vec(), (0..n).rev().collect_vec());
    }
}

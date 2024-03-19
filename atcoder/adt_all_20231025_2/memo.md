# i
順列 p,i から pが pre-order で i が in-order になるような気があるか判定する問題。
自分ではかなりきれいに書けたと思ってよかったのだが、 O(N^2) になっている（ `root_ind_i` を毎回探しているので）。
プログラムとしてはいい気がするので残しておく。

```rust
fn pre_order_and_in_order(
    p: Vec<usize>,
    i: Vec<usize>,
) -> Option<(Option<usize>, Vec<(Option<usize>, Option<usize>)>)> {
    let n = p.len();
    let mut nodes = vec![None; n];
    let head = pre_order_and_in_order_rec(&p, &i, &mut nodes)?;
    let nodes = nodes.into_iter().map(|nodes| nodes.unwrap()).collect();
    Some((head, nodes))
}

fn pre_order_and_in_order_rec(
    p: &[usize],
    i: &[usize],
    nodes: &mut [Option<(Option<usize>, Option<usize>)>],
) -> Option<Option<usize>> {
    let n = p.len();
    if n == 0 {
        return Some(None);
    }
    let root = p[0];
    let root_ind_i = i.iter().position(|&inode| root == inode)?;
    let left_head = pre_order_and_in_order_rec(&p[1..root_ind_i + 1], &i[0..root_ind_i], nodes)?;
    let right_head = pre_order_and_in_order_rec(&p[root_ind_i + 1..], &i[root_ind_i + 1..], nodes)?;
    nodes[root] = Some((left_head, right_head));
    Some(Some(root))
}
```

改良して find を渡せるようにしつつ、そのために持ちまわすものを添え字にした。
```rust
// p[pl..pr] と i[il..ir] を処理する pl - pr == il - ir =: len
fn pre_order_and_in_order_rec(
    (p, i): (&[usize], &[usize]),
    (pl, pr, il, ir): (usize, usize, usize, usize),
    find: &Vec<usize>,
    nodes: &mut [Option<(Option<usize>, Option<usize>)>],
) -> Option<Option<usize>> {
    debug_assert!(pr >= pl && ir >= il && pr - pl == ir - il);
    let len = pr - pl;
    if len == 0 {
        return Some(None);
    }
    let r = p[pl];
    let is = *find.get(r).filter(|&ind| (il..ir).contains(ind))?;
    // p[pl] p[pl+1..ps+1] p[ps+1..pr]
    // i[il..is] i[is] i[is+1..ir]
    let ps = pl + is - il;
    let left_head = pre_order_and_in_order_rec((p, i), (pl + 1, ps + 1, il, is), find, nodes)?;
    let right_head = pre_order_and_in_order_rec((p, i), (ps + 1, pr, is + 1, ir), find, nodes)?;
    nodes[r] = Some((left_head, right_head));
    Some(Some(r))
}
```

これは stack overflow になる。
確かに引数に参照が4つも入っているので重いかも。
これを変えたい。

use std::collections::VecDeque;

use im_rc::HashSet;
use itertools::Itertools;
use proconio::{fastout, marker::Usize1};

#[fastout]
fn main() {
    proconio::input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let ch = to_child_edge(uv, 0);
    // eprintln!("{ch:?}");
    for (l, r) in ranges_on_tree(ch, 0) {
        println!("{} {}", l, r);
    }
}

fn to_child_edge(uv: Vec<(usize, usize)>, parent: usize) -> Vec<Vec<usize>> {
    let n = uv.len() + 1;
    let mut ch = vec![HashSet::new(); n];
    for (u, v) in uv {
        ch[u].insert(v);
        ch[v].insert(u);
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for &v in &ch[parent] {
        queue.push_back((v, parent))
    }
    while let Some((v, p)) = queue.pop_front() {
        ch[v].remove(&p);
        for &c in &ch[v] {
            queue.push_back((c, v));
        }
    }
    ch.into_iter()
        .map(|set| set.into_iter().collect())
        .collect_vec()
}

fn ranges_on_tree(to_child: Vec<Vec<usize>>, parent: usize) -> Vec<(usize, usize)> {
    let n = to_child.len();
    let mut ranges: Vec<Option<(usize, usize)>> = vec![None; n];
    range_construct(&to_child, &mut ranges, parent, 1);
    ranges.into_iter().map(|range| range.unwrap()).collect()
}

// change: ranges[parent] = Some(l, r)
fn range_construct(
    to_child: &Vec<Vec<usize>>,
    ranges: &mut Vec<Option<(usize, usize)>>,
    parent: usize,
    start: usize,
) {
    if to_child[parent].is_empty() {
        ranges[parent] = Some((start, start));
        return;
    }

    let mut s = start;
    for &c in &to_child[parent] {
        range_construct(to_child, ranges, c, s);
        let (l, r) = ranges[c].unwrap();
        s = r + 1;
    }

    ranges[parent] = Some((start, s - 1));
}

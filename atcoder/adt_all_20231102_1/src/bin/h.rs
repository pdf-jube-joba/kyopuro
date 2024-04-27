use petgraph::unionfind::UnionFind;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize, e: usize,
        uv: [(Usize1, Usize1); e],
        q: usize,
        x: [Usize1; q],
    }
    for ans in blackout2((n, m), uv, x) {
        println!("{ans}");
    }
}

fn blackout2((n, m): (usize, usize), uv: Vec<(usize, usize)>, mut x: Vec<usize>) -> Vec<usize> {
    let uv: Vec<(usize, usize)> = uv
        .into_iter()
        .map(|(u, v)| (if u < n { u } else { n }, if v < n { v } else { n }))
        .collect();
    let mut uv_q: Vec<bool> = vec![true; uv.len()];
    let x: Vec<(usize, usize)> = x
        .into_iter()
        .rev()
        .map(|i| {
            uv_q[i] = false;
            uv[i]
        })
        .collect();

    let mut uf: UnionFind<usize> = UnionFind::new(n + 1);
    // size[i] == #{j | exists path from i to j} if i = uf.find(i)
    let mut size: Vec<usize> = vec![1; n + 1];

    for (i, (u, v)) in uv.into_iter().enumerate() {
        if !uv_q[i] {
            continue;
        }
        let (pu, pv) = (uf.find(u), uf.find(v));
        if pu == pv {
            continue;
        }

        uf.union(u, v);
        let p = uf.find(u);
        size[p] = size[pu] + size[pv];
    }

    let mut cur = size[uf.find(n)] - 1;

    let mut ans = vec![cur];

    for (u, v) in x {
        let (pu, pv) = (uf.find(u), uf.find(v));
        if pu != pv {
            if pu == uf.find(n) {
                cur += size[pv];
            } else if pv == uf.find(n) {
                cur += size[pu];
            }
            uf.union(u, v);
            let p = uf.find(u);
            size[p] = size[pu] + size[pv];
        }
        ans.push(cur);
    }

    ans.pop();
    ans.reverse();
    ans
}

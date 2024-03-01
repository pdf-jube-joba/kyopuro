use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    println!("{}", conn_comp_num(n, uv));
}

fn conn_comp_num(n: usize, edges: Vec<(usize, usize)>) -> usize {
    let mut uf = Find::new(n);
    for (u, v) in edges {
        uf.union((u, v));
    }
    let mut ps = (0..n).map(|i| uf.find(i)).collect_vec();
    ps.sort();
    ps.dedup();
    ps.len()
}

#[derive(Debug, Clone, PartialEq)]
struct Find {
    // Ok(p) ... parent is p | Err(e) ... parent of size e tree
    p: Vec<Result<usize, usize>>,
}

impl Find {
    fn new(size: usize) -> Self {
        Find {
            p: vec![Err(1); size],
        }
    }
    fn union(&mut self, (u, v): (usize, usize)) {
        let up = self.find(u);
        let up_size = self.p[up].unwrap_err();
        let vp = self.find(v);
        let vp_size = self.p[vp].unwrap_err();
        if up_size <= vp_size {
            self.p[up] = Ok(vp);
            self.p[vp] = Err(up_size + vp_size);
        } else {
            self.p[vp] = Ok(up);
            self.p[up] = Err(up_size + vp_size);
        }
    }
    fn find(&self, v: usize) -> usize {
        match self.p[v] {
            Ok(p) => {
                debug_assert!(v != p);
                self.find(p)
            },
            Err(_) => v,
        }
    }
}

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, q: usize,
    }
    let qs = (0..q)
        .map(|_| {
            proconio::input! {
                t: usize,
            }
            if t == 1 {
                proconio::input! {
                    u: Usize1, v: Usize1,
                }
                Q::Con(u, v)
            } else {
                proconio::input! {
                    u: Usize1,
                }
                Q::Del(u)
            }
        })
        .collect_vec();
}

enum Q {
    Con(usize, usize),
    Del(usize),
}

fn isolation(n: usize, qs: Vec<Q>) -> Vec<usize> {

    todo!()
}

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use num_traits::Saturating;
use std::cmp::*;

fn main() {
    proconio::input! {
        n: usize, h: usize,
        td: [(usize, usize); n],
    }
    println!("{}", damage_over_time(h, td));
}

// sum_{u in l..r} max(a * u, b)
fn integrate((a, b): (usize, usize), (l, r): (usize, usize)) -> usize {
    if a == 0 {
        return b.saturating_mul(r - l);
    }
    // sum_{u in l..r} b (saturating)
    let squ = |b: usize, (l, r): (usize, usize)| -> usize { b.saturating_mul(r.saturating_sub(l)) };
    // sum_{u in l..r} a * u (saturating)
    let tri = |a: usize, (l, r): (usize, usize)| -> usize {
        let s = l.saturating_add(r).saturating_sub(1);
        a.saturating_mul(s).saturating_mul(r.saturating_sub(l)) / 2
    };
    let i = b / a + 1;
    let squ = squ(b, (l, min(r, i)));
    let tri = tri(a, (max(l, i), r));
    squ + tri
}

// same as min {i in NN | sum_{u in 0..=i} max_{k in 0..N} d[k] * min(u, t[k]) >= H}
fn damage_over_time(h: usize, mut td: Vec<(usize, usize)>) -> usize {
    let td = {
        td.sort_by_key(|tdi| (Reverse(tdi.1), Reverse(tdi.0)));
        let mut new_td = vec![td[0]];
        for (t, d) in td {
            let (nt, nd) = new_td.last().unwrap();
            if nt * nd < t * d {
                new_td.push((t, d));
            }
        }
        new_td
    };
    let prevt = |i: usize| -> usize {
        if i == 0 {
            0
        } else {
            td[i - 1].0
        }
    };
    let prevtd = |i: usize| -> usize {
        if i == 0 {
            0
        } else {
            td[i - 1].1 * td[i - 1].0
        }
    };
    let n = td.len();
    let mut i = 0;
    // now = sum_{u in 0..t[i-1]} f(td)(u)
    let mut now = 0;
    let i = loop {
        // area = sum_{u in t[i-1]..t[i]} f(td)(u)
        let area = if i == 0 {
            integrate((td[0].1, prevtd(0)), (prevt(0), td[0].0))
        } else if i == n {
            std::usize::MAX
        } else {
            integrate((td[i].1, prevtd(i)), (prevt(i), td[i].0))
        };
        if now.saturating_add(area) >= h {
            break i;
        }
        now += area;
        i += 1;
    };
    // now < h <= now + sum_{u in t[i-1]..t[i]} f(td)(u)
    let (mut ng, mut ok) = (prevt(i), h);
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if now + integrate((td[i].1, prevtd(i)), (prevt(i), mid)) >= h {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i() {
        assert_eq!(integrate((1, 1), (1, 2)), 1);
        assert_eq!(integrate((1, 1), (1, 4)), 6);
        assert_eq!(integrate((2, 5), (0, 1)), 5);
        assert_eq!(integrate((2, 5), (0, 2)), 10);
        assert_eq!(integrate((2, 5), (1, 2)), 5);
        assert_eq!(integrate((2, 5), (1, 3)), 10);
        assert_eq!(integrate((2, 5), (1, 4)), 16);
        assert_eq!(integrate((2, 5), (1, 5)), 24);
        assert_eq!(integrate((2, 5), (2, 2)), 0);
        assert_eq!(integrate((2, 5), (2, 3)), 5);
        assert_eq!(integrate((2, 5), (2, 4)), 11);
        assert_eq!(integrate((2, 5), (2, 5)), 19);
        assert_eq!(integrate((2, 5), (3, 4)), 6);
        assert_eq!(integrate((2, 5), (3, 5)), 14);
        assert_eq!(integrate((2, 5), (3, 6)), 24);
    }
    #[test]
    fn t() {
        let v = vec![(1, 1), (1, 1)];
        assert_eq!(damage_over_time(1, v.clone()), 1);
        assert_eq!(damage_over_time(2, v.clone()), 2);
        assert_eq!(damage_over_time(3, v.clone()), 3);
        assert_eq!(damage_over_time(4, v.clone()), 4);

        let v = vec![(10, 1)];
        assert_eq!(damage_over_time(1, v.clone()), 1);
        assert_eq!(damage_over_time(2, v.clone()), 2);
        assert_eq!(damage_over_time(3, v.clone()), 2);
        assert_eq!(damage_over_time(4, v.clone()), 3);
        assert_eq!(damage_over_time(5, v.clone()), 3);
        assert_eq!(damage_over_time(6, v.clone()), 3);
        assert_eq!(damage_over_time(7, v.clone()), 4);

        assert_eq!(damage_over_time(95, v.clone()), 14);
        assert_eq!(damage_over_time(96, v.clone()), 15);
        assert_eq!(damage_over_time(100, v.clone()), 15);
        assert_eq!(damage_over_time(105, v.clone()), 15);

        let v = vec![(1, 2), (1, 1), (2, 1)];
        assert_eq!(damage_over_time(1, v.clone()), 1);
        assert_eq!(damage_over_time(2, v.clone()), 1);
        assert_eq!(damage_over_time(3, v.clone()), 2);
        assert_eq!(damage_over_time(4, v.clone()), 2);
        assert_eq!(damage_over_time(5, v.clone()), 3);
        assert_eq!(damage_over_time(6, v.clone()), 3);

        let v = vec![(1, 1), (3, 2), (2, 5), (4, 3), (5, 4)];
        assert_eq!(damage_over_time(1, v.clone()), 1);
        assert_eq!(damage_over_time(5, v.clone()), 1);
        assert_eq!(damage_over_time(15, v.clone()), 2);
        assert_eq!(damage_over_time(16, v.clone()), 3);
        assert_eq!(damage_over_time(27, v.clone()), 3);
        assert_eq!(damage_over_time(28, v.clone()), 4);

        let v = vec![(1000000000, 1)];
        assert_eq!(damage_over_time(1000000000000000000, v), 1500000000);
    }
}

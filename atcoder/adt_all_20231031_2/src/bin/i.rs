use itertools::Itertools;
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

// f(td: Vec<(usize, usize>)) = x |-> max_{k in 0..td.len()} d[k] * min(u, t[k])
struct TDMAX {
    td: Vec<(usize, usize)>,
    maxdf: Vec<(Option<usize>, Option<usize>)>,
}

impl TDMAX {
    fn new(td: Vec<(usize, usize)>) -> Self {
        let mut tdi = (0..td.len()).collect_vec();
        tdi.sort_by_key(|&i| (td[i].1, Reverse(td[i].0)));
        tdi.dedup_by_key(|i| td[*i].1);
        tdi.sort_by_key(|&i| (td[i].0, Reverse(td[i].1)));
        tdi.dedup_by_key(|i| td[*i].0);
        // i in tdi => forall j, t[i] = t[j] => d[i] > d[j], d[i] = d[j] => t[i] > t[j]
        // i in tdi < j in tdi => t[i] < t[j]
        let td = tdi.into_iter().map(|i| td[i]).collect_vec();
        let n = td.len();

        let dkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1).unwrap_or(0) };
        let tdkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1 * td[k].0).unwrap_or(0) };

        let t2 = {
            let mut t = vec![0, std::usize::MAX];
            t.extend(td.iter().map(|td| td.0));
            t.sort();
            t.dedup();
            t
        };
        let mut kd = {
            let mut v = (0..n).collect_vec();
            v.sort_by_key(|&i| td[i].1);
            v
        };

        let mut maxdf = vec![];
        let (mut maxdi, mut maxfi): (Option<usize>, Option<usize>) = (kd.pop(), None);
        
        for i in 0..t2.len()-1 {
            maxdf.push((maxdi, maxfi));
            let np = kd.pop();
            match np {
                Some(np) => {
                    if np >= maxdi.unwrap() {
                        if tdkref(maxdi) >= tdkref(maxfi) {
                            maxfi = maxdi.take();
                        }
                        maxdi = Some(np);
                    } else if dkref(Some(i)) >= dkref(maxfi) {
                        maxfi = Some(i);
                    }
                }
                None => {
                    if tdkref(maxdi) >= tdkref(maxfi) {
                        maxfi = maxdi.take();
                    }
                }
            }
        }
        eprintln!("{maxdf:?}");

        Self { td, maxdf }
    }
    // fn at(x: usize) -> usize {

    // }
}

// same as min {i in NN | sum_{u in 0..=i} max_{k in 0..n} d[k] * min(u, t[k]) >= H}
fn damage_over_time(h: usize, td: Vec<(usize, usize)>) -> usize {
    let mut tdi = (0..td.len()).collect_vec();
    tdi.sort_by_key(|&i| (td[i].1, Reverse(td[i].0)));
    tdi.dedup_by_key(|i| td[*i].1);
    tdi.sort_by_key(|&i| (td[i].0, Reverse(td[i].1)));
    tdi.dedup_by_key(|i| td[*i].0);
    // i in tdi => forall j, t[i] = t[j] => d[i] > d[j], d[i] = d[j] => t[i] > t[j]
    // i in tdi < j in tdi => t[i] < t[j]
    let td = tdi.into_iter().map(|i| td[i]).collect_vec();
    let n = td.len();

    let dkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1).unwrap_or(0) };
    let tdkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1 * td[k].0).unwrap_or(0) };

    let t2 = {
        let mut t = vec![0, std::usize::MAX];
        t.extend(td.iter().map(|td| td.0));
        t.sort();
        t.dedup();
        t
    };
    let mut i = 0;
    let mut kd = {
        let mut v = (0..n).collect_vec();
        v.sort_by_key(|&i| td[i].1);
        v
    };
    // maxdi = argmax (k in 0..n | t2[i] <= t[k]) |-> d[k]
    // maxfi = argmax (k in 0..n | t[k] < t2[i]) |-> t[k] * d[k]
    let (mut maxdi, mut maxfi): (Option<usize>, Option<usize>) = (kd.pop(), None);
    // now_h == sum_{u in 0..t2[i]} max_{k in 0..n} d[k] * min(u, t[k])
    let mut now_h: usize = 0;

    let _ = TDMAX::new(td.clone());

    loop {
        // now_h < h
        let this_range_sum = integrate((dkref(maxdi), tdkref(maxfi)), (t2[i], t2[i + 1]));

        if h <= now_h.saturating_add(this_range_sum) {
            break;
        }

        now_h += this_range_sum;
        i += 1;
        let np = kd.pop();

        match np {
            Some(np) => {
                if np >= maxdi.unwrap() {
                    if tdkref(maxdi) >= tdkref(maxfi) {
                        maxfi = maxdi.take();
                    }
                    maxdi = Some(np);
                } else if dkref(Some(np)) >= dkref(maxfi) {
                    maxfi = Some(np);
                }
            }
            None => {
                if tdkref(maxdi) >= tdkref(maxfi) {
                    maxfi = maxdi.take();
                }
            }
        }
    }
    // now_h < h <= sum_{u in t2[i]..t2[i+1]}

    let from = t2[i];
    let (mut ng, mut ok) = (t2[i], t2[i + 1]);
    while ok - ng > 1 {
        let mid = if ok == std::usize::MAX {
            ng * 2
        } else {
            (ng + ok) / 2
        };
        if h <= now_h.saturating_add(integrate((dkref(maxdi), tdkref(maxfi)), (from, mid))) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    eprintln!("{i} {maxdi:?} {maxfi:?} {ng} {ok}");
    ok - 1
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
        // let v = vec![(1, 1), (1, 1)];
        // assert_eq!(damage_over_time(1, v.clone()), 1);
        // assert_eq!(damage_over_time(2, v.clone()), 2);
        // assert_eq!(damage_over_time(3, v.clone()), 3);
        // assert_eq!(damage_over_time(4, v.clone()), 4);

        // let v = vec![(1, 2), (1, 2), (2, 1)];
        // assert_eq!(damage_over_time(1, v.clone()), 1);
        // assert_eq!(damage_over_time(2, v.clone()), 1);
        // assert_eq!(damage_over_time(3, v.clone()), 2);
        // assert_eq!(damage_over_time(4, v.clone()), 2);
        // assert_eq!(damage_over_time(5, v.clone()), 3);
        // assert_eq!(damage_over_time(6, v.clone()), 3);

        let v = vec![(1, 1), (3, 2), (2, 5), (4, 3), (5, 4)];
        // assert_eq!(damage_over_time(1, v.clone()), 1);
        // assert_eq!(damage_over_time(5, v.clone()), 1);
        // assert_eq!(damage_over_time(15, v.clone()), 2);
        assert_eq!(damage_over_time(16, v.clone()), 3);
        // assert_eq!(damage_over_time(27, v.clone()), 3);
        // assert_eq!(damage_over_time(28, v.clone()), 4);
    }
}

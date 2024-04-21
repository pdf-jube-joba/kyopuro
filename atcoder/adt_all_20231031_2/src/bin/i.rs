use itertools::Itertools;

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
    let i = b / a;
    let sq = b.saturating_mul(std::cmp::min(r, i).saturating_sub(l));
    let tri = a
        .saturating_mul(i.saturating_add(r - 1))
        .saturating_mul(r.saturating_sub(std::cmp::max(l, i)))
        / 2;
    sq + tri
}

// same as min {i in NN | sum_{u in 0..=i} max_{k in 0..n} d[k] * min(u, t[k]) >= H}
fn damage_over_time(h: usize, td: Vec<(usize, usize)>) -> usize {
    let mut tdi = (0..td.len()).collect_vec();
    tdi.sort_by_key(|&i| (td[i].1, std::cmp::Reverse(td[i].0)));
    tdi.dedup_by_key(|i| td[*i].1);
    tdi.sort_by_key(|&i| (td[i].0, std::cmp::Reverse(td[i].1)));
    tdi.dedup_by_key(|i| td[*i].0);
    // i in tdi => forall j, t[i] = t[j] => d[i] > d[j], d[i] = d[j] => t[i] > t[j]
    // i in tdi < j in tdi => t[i] < t[j]
    let td = tdi.into_iter().map(|i| td[i]).collect_vec();
    let n = td.len();
    eprintln!("{td:?}");

    let dkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1).unwrap_or(0) };
    let tdkref = |k: Option<usize>| -> usize { k.map(|k| td[k].1 * td[k].0).unwrap_or(0) };

    let t2 = {
        let mut t = vec![0, h, h + 1];
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
    eprintln!("{kd:?}");
    // maxdi = argmax (k in 0..n | t2[i] <= t[k]) |-> d[k]
    // maxfi = argmax (k in 0..n | t[k] < t2[i]) |-> t[k] * d[k]
    let (mut maxdi, mut maxfi): (Option<usize>, Option<usize>) = (kd.pop(), None);
    // now_h == sum_{u in 0..t2[i]} max_{k in 0..n} d[k] * min(u, t[k])
    let mut now_h = 0;

    loop {
        // now_h + sum_{u in t2[i]..t2[i+1]} <= h
        let this_range_sum = integrate((dkref(maxdi), tdkref(maxfi)), (t2[i], t2[i + 1]));
        eprintln!(
            "{i} {maxdi:?} {:?} {maxfi:?} {:?} [{:?}, {:?}) {this_range_sum:?}",
            dkref(maxdi),
            tdkref(maxfi),
            t2[i],
            t2[i + 1],
        );

        if h <= now_h + this_range_sum {
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

    let from = t2[i];
    let (mut ng, mut ok) = (t2[i], t2[i + 1]);
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if h <= now_h + integrate((dkref(maxdi), tdkref(maxfi)), (from, mid)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        let r = damage_over_time(1, vec![(1, 1), (1, 1)]);
        assert_eq!(r, 1);
    }
}

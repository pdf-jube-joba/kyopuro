use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, h: usize,
        td: [(usize, usize); n],
    }
    println!("{}", damage_over_time(h, td));
}

// sum_{u in l..r} max(a * u, b) // r <= h then result = MAX
fn integrate((a, b): (usize, usize), (l, r): (usize, usize)) -> usize {
    if a == 0 {
        return b * (r - l);
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
    let n = td.len();
    let dkref = |k: Option<usize>| -> usize {
        if let Some(k) = k {
            td[k].1
        } else {
            0
        }
    };
    let tdkref = |k: Option<usize>| -> usize {
        if let Some(k) = k {
            td[k].1 * td[k].0
        } else {
            0
        }
    };
    let t2 = {
        let mut t = vec![0, h];
        t.extend(td.iter().map(|td| td.0));
        t.sort();
        t
    };
    let mut i = 0;
    let mut kd = {
        let mut v = (0..n).collect_vec();
        v.sort_by_key(|i| td[*i].1);
        v
    };
    // maxdi = argmax (k in 0..n | t2[i] <= t[k]) |-> d[k]
    // maxfi = argmax (k in 0..n | t[k] < t2[i]) |-> t[k] * d[k]
    let (mut maxdi, mut maxfi): (Option<usize>, Option<usize>) = (kd.pop(), None);
    // now_h == sum_{u in 0..t2[i]} max_{k in 0..n} d[k] * min(u, t[k])
    let mut now_h = 0;

    loop {
        eprintln!("{:?} {:?}", maxdi, maxfi);
        // now_h + sum_{u in t2[i]..t2[i+1]} <= h
        let this_range_sum = integrate((dkref(maxdi), tdkref(maxfi)), (t2[i], t2[i + 1]));

        debug_assert!(
            (maxdi.is_some() || t2[i + 1] == std::usize::MAX) && (maxfi.is_some() || t2[i] == 0)
        );

        if h <= now_h + this_range_sum {
            break;
        }

        now_h += this_range_sum;
        i += 1;
        let np = kd.pop();

        if let Some(np) = np {
            if np >= maxdi.unwrap() {
                if tdkref(maxdi) >= tdkref(maxfi) {
                    maxfi = maxdi.take();
                }
                maxdi = Some(np);
            } else if dkref(Some(np)) >= dkref(maxfi) {
                maxfi = Some(np);
            };
        } else if tdkref(maxdi) >= tdkref(maxfi) {
            maxfi = maxdi.take();
        }
    }

    let (mut ng, mut ok) = (t2[i], t2[i + 1]);
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if h <= now_h + integrate((dkref(maxdi), tdkref(maxfi)), (ng, mid)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

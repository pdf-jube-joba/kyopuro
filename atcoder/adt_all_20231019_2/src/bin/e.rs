fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!(
        "{}",
        if let Some(u) = sum_max(a) {
            u.to_string()
        } else {
            "-1".to_owned()
        }
    )
}

fn sum_max(a: Vec<usize>) -> Option<usize> {
    let mut ev = vec![];
    let mut od = vec![];
    for ai in a {
        if ai % 2 == 0 {
            ev.push(ai);
        } else {
            od.push(ai);
        }
    }
    ev.sort();
    od.sort();
    match (ev.pop(), ev.pop(), od.pop(), od.pop()) {
        (Some(e1), Some(e2), Some(o1), Some(o2)) => Some(std::cmp::max(e1 + e2, o1 + o2)),
        (Some(v1), Some(v2), _, _) | (_, _, Some(v1), Some(v2)) => Some(v1 + v2),
        _ => None,
    }
}

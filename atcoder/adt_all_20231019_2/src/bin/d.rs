use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize, t: usize,
        a: [usize; n-1],
        xy: [(Usize1, usize); m],
    }
    println!("{}", if exploreable(t, a, xy) { "Yes" } else { "No" })
}

fn exploreable(mut t: usize, a: Vec<usize>, xy: Vec<(usize, usize)>) -> bool {
    let n = a.len() + 1;
    let mut bonus = vec![0; n];
    for (x, y) in xy {
        bonus[x] = y;
    }

    for i in 0..n - 1 {
        // move i room -> i+1 room
        match t.checked_sub(a[i]) {
            Some(t1) if t1 != 0 => {
                t = t1;
            }
            _ => {
                return false;
            }
        };
        // get bonus of i+1 room
        t += bonus[i + 1];
    }

    true
}

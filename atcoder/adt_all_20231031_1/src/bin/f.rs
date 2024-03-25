use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, a: Usize1, b: Usize1,
        p: Usize1, q: Usize1, r: Usize1, s: Usize1,
    }
    let ans = x_drawing(n, (a, b), (p, q, r, s));
    for ans in ans {
        println!(
            "{}",
            ans.into_iter().map(|b| if b { '#' } else { '.' }).join("")
        )
    }
}

fn x_drawing(
    n: usize,
    (a, b): (usize, usize),
    (p, q, r, s): (usize, usize, usize, usize),
) -> Vec<Vec<bool>> {

    todo!()
}

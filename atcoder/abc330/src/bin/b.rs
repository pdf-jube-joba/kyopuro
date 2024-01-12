use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, l: usize, r: usize,
        a: [usize; n],
    }
    let str: String = a.into_iter().map(|ai| format!("{}", xi(l, r, ai))).join(" ");
    println!("{}", str);
}

fn xi(l: usize, r: usize, a: usize) -> usize {
    if a <= l {
        l
    } else if r <= a {
        r
    } else {
        a
    }
}

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};

const ABC: [char; 3] = ['A', 'B', 'C'];

fn main() {
    proconio::input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }
    for ans in abc_transform(s, tk) {
        println!("{}", ans)
    }
}

fn abc_transform(s: Vec<char>, tk: Vec<(usize, usize)>) -> Vec<char> {
    tk.into_iter().map(|(t, k)| f(&s, (t, k))).collect_vec()
}

fn f(s: &Vec<char>, (t, k): (usize, usize)) -> char {
    if t == 0 {
        s[k]
    } else if k == 0 {
        next(s[0], t)
    } else {
        next(f(s, (t - 1, k / 2)), k % 2 + 1)
    }
}

fn next(c: char, s: usize) -> char {
    let i = ABC.iter().position(|&c0| c0 == c).unwrap();
    ABC[(i + s) % ABC.len()]
}

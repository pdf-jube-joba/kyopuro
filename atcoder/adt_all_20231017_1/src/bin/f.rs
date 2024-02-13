use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::marker::Chars;
enum Move {
    R,
    L,
    U,
    D,
}

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|c| match c {
            'R' => Move::R,
            'L' => Move::L,
            'U' => Move::U,
            'D' => Move::D,
            _ => unreachable!(),
        })
        .collect_vec();

    println!("{}", if f(s) { "Yes" } else { "No" })
}

fn f(s: Vec<Move>) -> bool {
    let mut set: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut now: (isize, isize) = (0, 0);

    set.insert(now);

    for m in s {
        match m {
            Move::R => {
                now.1 += 1;
            }
            Move::L => {
                now.1 -= 1;
            }
            Move::U => {
                now.0 += 1;
            }
            Move::D => {
                now.0 -= 1;
            }
        }
        if !set.insert(now) {
            return true;
        }
    }

    false
}

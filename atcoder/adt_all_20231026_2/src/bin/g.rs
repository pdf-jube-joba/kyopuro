use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, x: usize,
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|c| match c {
            'U' => M::U,
            'R' => M::R,
            'L' => M::L,
            _ => unreachable!(),
        })
        .collect_vec();
    println!("{}", moves_on_binary_tree(x, s));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum M {
    U,
    L,
    R,
}

fn moves_on_binary_tree(mut x: usize, s: Vec<M>) -> usize {
    let mut stack: Vec<M> = vec![];
    for si in s {
        match si {
            M::U if stack.last().filter(|v| matches!(v, M::L | M::R)).is_some() => {
                stack.pop();
            }
            _ => {
                stack.push(si);
            }
        }
    }
    for si in stack {
        match si {
            M::U => {
                x /= 2;
            }
            M::R => {
                x = 2 * x + 1;
            }
            M::L => {
                x = 2 * x;
            }
        }
    }
    x
}

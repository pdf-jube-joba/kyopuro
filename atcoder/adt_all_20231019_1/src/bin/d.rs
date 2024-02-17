use itertools::{iproduct, Itertools};
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|char| match char {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect_vec();

    println!("{}", if is_split(s) { "Yes" } else { "No" })
}

// s[i] <=> i-th pin is knocked down
fn is_split(s: Vec<bool>) -> bool {
    if s[0] {
        return false;
    }

    let lane: Vec<Vec<usize>> = vec![
        vec![6],
        vec![3],
        vec![7, 1],
        vec![4, 0],
        vec![8, 2],
        vec![5],
        vec![9],
    ];
    let lane_len = lane.len();
    let i_lane_knock_down = |i: usize| -> bool { lane[i].iter().all(|si| !s[*si]) };

    for i in 0..lane_len {
        eprintln!("{i} {}", i_lane_knock_down(i))
    }

    iproduct!(0..7, 0..lane_len)
        .filter(|&(i, j)| i < j)
        .any(|(i, j)| {
            !i_lane_knock_down(i)
                && !i_lane_knock_down(j)
                && (i + 1..j).any(|k| i_lane_knock_down(k))
        })
}

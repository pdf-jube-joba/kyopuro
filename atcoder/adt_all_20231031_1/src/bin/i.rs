use ac_library::ModInt998244353 as ModInt;
use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|si| {
            si.into_iter().map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            })
        })
        .collect_vec();
}

fn yet_another_grid_task(s: Vec<Vec<char>>) -> ModInt {
    
    todo!()
}

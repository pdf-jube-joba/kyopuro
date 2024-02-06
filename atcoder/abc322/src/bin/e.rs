use im_rc::HashMap;

fn main() {
    proconio::input! {
        n: usize, k: usize, p: usize,
        c: [(usize, [usize; k+1]); n]
    }
}

fn convert_to_base(k: usize, ps: Vec<usize>) -> usize {
    ps.into_iter()
        .enumerate()
        .map(|(i, pi)| pi * k.pow(i as u32))
        .sum()
}

fn convert_from_base(k: usize, mut p: usize) -> Vec<usize> {
    let mut ps = vec![];
    while p != 0 {
        ps.push(p % k);
        p /= k;
    }
    ps
}

fn min_cost(k: usize, p: usize, c: Vec<(usize, Vec<usize>)>) -> Option<usize> {
    let mut dp: Vec<Vec<usize>> = vec![];
    todo!()
}

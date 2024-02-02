use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, x: usize, y: usize,
        pt: [(usize, usize); n-1],
        q: usize,
        qi: [usize; q],
    }

    let dp = pre(pt);
    let lcm = dp.len();
    for qi in qi {
        let time = (x + qi) + dp[(x + qi) % lcm] + y;
        println!("{}", time);
    }
}

fn pre(pt: Vec<(usize, usize)>) -> Vec<usize> {
    let lcm: usize = pt
        .iter()
        .map(|(pi, ti)| *pi)
        .reduce(num::integer::lcm)
        .unwrap();
    // dp[i in 0..lcm] = the time it takes to arrive at last stop when start at i time from 1 st stop
    let mut dp = (0..lcm).collect_vec();
    for &(pi, ti) in &pt {
        for time in &mut dp {
            *time = num::integer::div_ceil(*time, pi) * pi + ti;
        }
    }
    for i in 0..lcm {
        dp[i] -= i;
    }
    dp
}
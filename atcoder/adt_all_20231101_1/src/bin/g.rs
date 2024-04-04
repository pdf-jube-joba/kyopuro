fn main() {
    proconio::input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    println!("{}", project_planning(k, a));
}

fn project_planning(k: usize, a: Vec<usize>) -> usize {
    let mut ok = 0;
    let mut ng = 10usize.pow(18) / k;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;
        let sum: u128 = a.iter().map(|ai| std::cmp::min(*ai, md) as u128).sum();
        if sum >= (k * md) as u128 {
            ok = md
        } else {
            ng = md
        }
    }
    ok
}

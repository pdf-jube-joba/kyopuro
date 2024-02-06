use ac_library::ModInt998244353 as MInt;

fn main() {
    proconio::input! {
        n: usize, x: usize,
        t: [usize; n],
    }
    println!("{}", pos(t, x))
}

fn pos(tis: Vec<usize>, x: usize) -> MInt {
    let n = tis.len();
    // p[t] = new music start at t
    let mut p: Vec<MInt> = vec![MInt::new(1)];
    for t in 1..=x {
        let new = tis
            .iter()
            .map(|&ti| if t < ti { MInt::new(0) } else { p[t - ti] })
            .sum::<MInt>()
            / n;
        // p[t] = new;
        p.push(new);
    }
    if tis[0] < x + 1 {
        ((x + 1 - tis[0])..=x).map(|t| p[t]).sum::<MInt>() / n
    } else {
        (0..=x).map(|t| p[t]).sum::<MInt>() / n
    }
}

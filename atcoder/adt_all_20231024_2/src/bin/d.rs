fn main() {
    proconio::input! {
        n: usize, a: [usize; n],
    }
    println!("{}", max_angle(a))
}

fn max_angle(a: Vec<usize>) -> usize {
    let mut v: Vec<usize> = vec![0];
    for ai in a {
        for vi in v.iter_mut() {
            *vi += ai;
            *vi %= 360;
        }
        v.push(0);
    }
    let n = v.len();
    // eprintln!("{:?}", v);
    v.sort();
    (0..n).map(|i| (v[(i + 1) % n] + 360 - v[i % n]) % 360).max().unwrap()
}

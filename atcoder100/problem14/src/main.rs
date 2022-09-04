use std::vec;

fn main() {
    proconio::input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    println!("{}", minimize(&a, k));

}

fn convert(n: usize, a: i32) -> Vec<bool> {
    let mut vec = Vec::with_capacity(n);
    let mut t = a;
    for _ in 0..n {
        vec.push(t % 2 != 0);
        t = t / 2;
    }
    vec
}

fn minimize(a: &[usize], k: usize) -> usize {
    let n = a.len();
    let mut min = usize::MAX;
    for bits in (0..(1 << n)).map(|i|{convert(n, i)}) {
        let mut b: Vec<usize> = vec![0; n];
        let mut now = 0;
        if bits.iter().filter(|b|**b).count() < k {continue;}
        for i in 0..n {
            if bits[i] && now > a[i] {
                b[i] = now - a[i] + 1;
            }
            now = std::cmp::max(now, a[i] + b[i]);
        }
        let sum = b.iter().fold(0, |acc, i|{acc + i});
        println!("{}", sum);
        min = std::cmp::min(min, sum);
    }
    min
}

#[test]
fn minimize_test_1() {
    let a = vec![3949, 3774, 3598, 3469, 3424];
    let k = 5;
    assert_eq!(minimize(&a, k), 1541);
}

#[test]
fn minimize_test_2() {
    let a = vec![7,4,2,6,4];
    let k = 3;
    assert_eq!(minimize(&a, k), 7);
}
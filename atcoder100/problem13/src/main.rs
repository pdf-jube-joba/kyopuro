use std::vec;

fn main() {

    proconio::input! {
        r: usize, c: usize,
        a: [[usize; c]; r],
    }

    let a: Vec<Vec<bool>> = a.into_iter().map(|vec|{
        vec.into_iter().map(|v|{v != 0}).collect()
    }).collect();

println!("{}", maximize(&a));

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

fn maximize(a: &[Vec<bool>]) -> usize {
    let r = a.len();
    let c = a[0].len(); // a[i].len() == a[j].len()
    let mut max = 0;
    for bits in (0..(1 << r)).map(|i|(convert(r, i))) {
        let a: Vec<Vec<bool>> = a.into_iter().enumerate().map(|(i, vec)|{
            vec.into_iter().map(|j|{bits[i] ^ j}).collect()
        }).collect();
        let mut sum = 0;
        for i in 0..c {
            let count = (0..r).filter(|j|{a[*j][i]}).count();
            sum += std::cmp::max(count, r - count);
        }
        max = std::cmp::max(max, sum);
    }
    max
}

#[test]
fn maximize_test_1 () {
    let a = vec![
        vec![false, true, false, true, false],
        vec![true, false, false, false, true],
    ];
    assert_eq!(maximize(&a), 9)
}

#[test]
fn maximize_test_2 () {
    let a = vec![
        vec![true, false, false, false, true , false],
        vec![true, true , true , false, true , false],
        vec![true, false, true , true , false, true ],
    ];
    assert_eq!(maximize(&a), 15)
}
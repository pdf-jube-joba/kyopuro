fn main() {
    proconio::input!{
        n: usize, m: usize,
        s: [[usize]; m],
        p: [usize; m],
    }

    println!("{}", count(n, &s, &p))

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

#[test]
fn convert_test () {
    assert_eq!(convert(1, 0), &[false]);
    assert_eq!(convert(1, 1), &[true]);
    assert_eq!(convert(2, 0), &[false, false]);
    assert_eq!(convert(2, 1), &[true, false]);
    assert_eq!(convert(2, 2), &[false, true]);
    assert_eq!(convert(2, 3), &[true, true]);
    assert_eq!(convert(2, 4), &[false, false]);
}

fn count(n: usize, s: &[Vec<usize>], p: &[usize]) -> usize {
    let m = s.len();
    let mut count = 0;
    for bits in (0..(1 << n)).map(|i|convert(n, i)) {
        let switch_pushed: Vec<usize> = s.iter().map(|switches|{
            switches.iter().filter(|switch|{bits[**switch-1]}).count()
        }).collect();
        if (0..m).all(|i|{switch_pushed[i] % 2 == p[i]}) {
            count += 1;
        }
    }
    count
}

#[test]
fn count_test_1() {
    let n = 2;
    let s = vec![vec![1,2], vec![2]];
    let p = vec![0, 1];
    assert_eq!(count(n, &s, &p), 1)
}

#[test]
fn count_test_2() {
    let n = 2;
    let s = vec![vec![1,2], vec![2], vec![2]];
    let p = vec![0, 0, 1];
    assert_eq!(count(n, &s, &p), 0)
}

#[test]
fn count_test_3() {
    let n = 5;
    let s = vec![vec![1,2,5], vec![2,3]];
    let p = vec![1, 0];
    assert_eq!(count(n, &s, &p), 8)
}
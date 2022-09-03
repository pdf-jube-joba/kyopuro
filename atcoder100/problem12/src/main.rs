fn main() {
    proconio::input!{
        n: usize, m: usize,
        rel: [(usize, usize); m],
    }

    println!("{}", count(n, &rel));

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

fn count(n: usize, rel: &[(usize, usize)]) -> usize {
    println!("{:?}", rel);
    let mut count = 1;
    for bits in (0..(1 << n)).map(|i| convert(n, i)) {
        let all: Vec<(usize, usize)> = (0..n).map(|i|{
            (i+1..n).map(move |x|{(i, x)})
        }).flatten().filter(|(i,j)|{bits[*i] && bits[*j]}).collect();
        if all.iter().all(|(i,j)|{
            rel.iter().any(|(x,y)|{*x - 1 == *i && *y - 1 == *j})
        }) {
            count = std::cmp::max(count, bits.iter().filter(|b|**b).count());
        }
    }
    count
}

#[test]
fn count_test_1() {
    let n = 5;
    let m = 3;
    let rel = vec![(1,2), (2,3), (1,3)];
    assert_eq!(count(n, &rel), 3)
}

#[test]
fn count_test_2() {
    let n = 5;
    let m = 3;
    let rel = vec![(1,2), (2,3), (3,4)];
    assert_eq!(count(n, &rel), 2)
}

#[test]
fn count_test_3() {
    let n = 7;
    let m = 9;
    let rel = vec![(1,2), (1,3), (2,3), (4,5), (4,6), (4,7), (5,6), (5,7), (6,7)];
    assert_eq!(count(n, &rel), 4)
}
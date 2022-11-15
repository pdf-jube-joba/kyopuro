fn main() {
    proconio::input!{
        n: usize, m: usize,
        s: [[usize]; m],
        p: [usize; m],
    }

    println!("{}", count(n, &s, &p))

}

fn count(n: usize, s: &[Vec<usize>], p: &[usize]) -> usize {
    fn convert(n: usize, a: usize) -> Vec<bool> {
        (0..n).map(|i|{
            a & (1 << i) != 0
        }).collect()
    }
    let m = s.len();
    (0..(1 << n))
    .map(|i|convert(n, i))
    .filter(|bits|{
        (0..m).all(|i|{
            s[i].iter().filter(|switch|bits[**switch-1]).count() % 2 == p[i]
        })
    }).count()
}

#[cfg(test)]
mod tests {
    use crate::*;
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
}
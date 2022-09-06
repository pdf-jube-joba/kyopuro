fn main() {
    proconio::input!{
        n: usize, m: usize,
        rel: [(usize, usize); m],
    }

    println!("{}", count(n, &rel));

}

fn count(n: usize, rel: &[(usize, usize)]) -> usize {
    fn convert(n: usize, a: usize) -> Vec<bool> {
        (0..n).map(|i|{
            a & (1 << i) != 0
        }).collect()
    }
    (0..(1 << n)).map(|i| convert(n, i)).filter_map(|bits|{
        let all: Vec<(usize, usize)> = (0..n).map(|i|{
            (i+1..n).map(move |x|{(i, x)})
        }).flatten().filter(|(i,j)|{bits[*i] && bits[*j]}).collect();
        let result = all.iter().all(|(i,j)|{
            rel.iter().any(|(x,y)|{*x - 1 == *i && *y - 1 == *j})
        });
        if result {Some(bits.iter().filter(|b|**b).count())} else {None}
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn count_test_1() {
        let n = 5;
        let _m = 3;
        let rel = vec![(1,2), (2,3), (1,3)];
        assert_eq!(count(n, &rel), 3)
    }

    #[test]
    fn count_test_2() {
        let n = 5;
        let _m = 3;
        let rel = vec![(1,2), (2,3), (3,4)];
        assert_eq!(count(n, &rel), 2)
    }

    #[test]
    fn count_test_3() {
        let n = 7;
        let _m = 9;
        let rel = vec![(1,2), (1,3), (2,3), (4,5), (4,6), (4,7), (5,6), (5,7), (6,7)];
        assert_eq!(count(n, &rel), 4)
    }
}
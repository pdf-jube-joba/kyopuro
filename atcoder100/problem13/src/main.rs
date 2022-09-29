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

fn maximize(a: &[Vec<bool>]) -> usize {
    fn convert(n: usize, a: usize) -> Vec<bool> {
        (0..n).map(|i|{
            a & (1 << i) != 0
        }).collect()
    }
    let r = a.len();
    let c = a[0].len(); // a[i].len() == a[j].len()
    (0..(1 << r))
    .map(|i|(convert(r, i)))
    .map(|bits|{
        (0..c).map(|j|{
            let count = (0..r)
                .filter(|&i| a[i][j] ^ bits[i])
                .count();
            std::cmp::max(count, r - count)
        }).sum()
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
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
}
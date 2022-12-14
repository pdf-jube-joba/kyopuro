fn main() {
    proconio::input!{
        n: usize, m: usize,
        a: [[usize; m]; n]
    }

    println!("{}", maximize(&a))

}

fn maximize(a: &[Vec<usize>]) -> usize {
    let n = a.len();
    let m = a[0].len();
    (0..m)
    .flat_map(|i|{
        (i..m).map(move |j| (i,j))
    })
    .map(|(i,j)|{
        (0..n).map(|k| std::cmp::max(a[k][i], a[k][j])).sum()
    })
    .max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn maximize_test_1(){
        let a = vec![
            vec![80, 84],
        ];
        assert_eq!(maximize(&a), 84);
    }
    #[test]
    fn maximize_test_2(){
        let a = vec![
            vec![37, 29, 70, 41],
            vec![85, 69, 76, 50],
            vec![53, 10, 95, 100],
        ];
        assert_eq!(maximize(&a), 250);
    }
    #[test]
    fn maximize_test_3(){
        let a = vec![
            vec![31, 41],
            vec![59, 26],
            vec![53, 58],
            vec![97, 93],
            vec![23, 84],
            vec![62, 64],
            vec![33, 83],
            vec![27, 95],
        ];
        assert_eq!(maximize(&a), 581);
    }
}
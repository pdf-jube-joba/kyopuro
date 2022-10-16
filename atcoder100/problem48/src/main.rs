fn main() {
    proconio::input! {
        n: usize,
        w: [usize; n],
    }

    println!("{}", maximize(&w) )
}

fn maximize(w: &[usize]) -> usize {
    let n = w.len();
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; n+1]; n+1];

    let f = |i: usize, j: usize, dp: &Vec<Vec<Option<usize>>>|{
        if i.abs_diff(j) <= 1 {0}
        else if dp[i+1][j-1].unwrap() == j-i-2 && w[i].abs_diff(w[j-1]) <= 1 {j-i}
        else {(i+1..j).map(|k| dp[i][k].unwrap() + dp[k][j].unwrap()).max().unwrap()}
    };

    let iter =
    (0..=n).flat_map(|h|{
        (0..=(n-h)).map(move |i|(i, i+h))
    });

    for (i,j) in iter {
        dp[i][j] = Some(f(i,j,&dp)) ;
    }

    dp[0][n].unwrap()

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn maximize_test(){
        let w = vec![1,2,3,4];
        assert_eq!(maximize(&w), 4);
        let w = vec![1,2,3,1];
        assert_eq!(maximize(&w), 4);
        let w = vec![5,1,2,3,6];
        assert_eq!(maximize(&w), 2);
        let w = vec![8,7,1,4,3,5,4,1,6,8,10,4,6,5];
        assert_eq!(maximize(&w), 12);
    }
}
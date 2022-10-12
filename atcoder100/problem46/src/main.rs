fn main() {
    proconio::input! {
        n: usize,
        in1: [(usize, usize); n],
    }
    let mut rc: Vec<usize> = in1.iter().map(|&(r,_)| r).collect();
    rc.extend([in1.last().unwrap().1]);
    
    println!("{}", minimize(&rc));
}

fn minimize(rc: &[usize]) -> usize {
    let n = rc.len() - 1;
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; n+1]; n+1];
    
    let f = |i,j, dp: &Vec<Vec<Option<usize>>>| {
        if i == j {return 0;}
        (i..j).map(|k: usize|{
            dp[i][k].unwrap() + dp[k+1][j].unwrap() + rc[i-1] * rc[k] * rc[j]
        }).min().unwrap()
    };

    let iter =
    (0..n).flat_map(|h|{
        (1..=(n-h)).map(move |i|(i, i+h))
    });

    for (i,j) in iter {
        dp[i][j] = Some(f(i, j, &dp));
    }

    dp[1][n].unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test(){
        let rc = vec![30, 35, 15, 5, 10, 20, 25];
        assert_eq!(minimize(&rc), 15125);
    }
}
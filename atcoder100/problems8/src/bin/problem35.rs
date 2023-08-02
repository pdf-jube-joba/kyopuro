fn main() {
    proconio::input! {
        n: usize, w: usize,
        vw: [(usize, usize); n],
    }

    println!("{}", nap(&vw, w));

}

fn nap(vw: &[(usize, usize)], w: usize) -> usize {
    let n = vw.len();
    let mut dp = vec![vec![None; w + 1]; n];
    for (i, &(vi, wi)) in vw.iter().enumerate() {
        for j in 0..=w {
            dp[i][j] = Some(std::cmp::max(
               if i > 0 {dp[i-1][j].unwrap()} else {0},
               if j >= wi {dp[i][j-wi].unwrap() + vi} else {0}
            ));
        }
    }

    dp[n-1][w].unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nap_test(){
        let vw = vec![(4,2), (5,2), (2,1), (8,3)];
        let w = 5;
        assert_eq!(nap(&vw, w), 13)
    }
}
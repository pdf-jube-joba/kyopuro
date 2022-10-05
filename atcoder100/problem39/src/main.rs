fn main() {
    proconio::input! {
        n: usize,
        c: [usize; n],
    }
    println!("{}", count(&c[0..c.len()-1], *c.last().unwrap()));
}

fn count(c: &[usize], b: usize) -> usize {
    let c_len = c.len();
    let mut dp = vec![vec![None; 20+1]; c_len];
    
    for j in 0..=20 {
        dp[0][j] = Some(0);
    }
    dp[0][c[0]] = Some(1);

    for i in 0..c_len-1 {
        for j in 0..=20 {
            let c = c[i+1];
            dp[i+1][j] = Some(
                if j >= c {dp[i][j-c].unwrap()} else {0}
                + if j + c <= 20 {dp[i][j+c].unwrap()} else {0}
            );
        }
    }

    dp[c_len-1][b].unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_test(){
        let b = 8;
        let c = vec![8,3,2,4,8,7,2,4,0,8];
        assert_eq!(count(&c, b), 10);
    }
}
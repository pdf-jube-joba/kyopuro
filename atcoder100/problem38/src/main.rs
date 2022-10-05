fn main() {
    proconio::input! {
        q: usize,
    }
    for _ in 0..q {
        proconio::input! {
            x: String,
            y: String,
        }

    }
}

fn max_len(x: &[u8], y: &[u8]) -> usize {
    let x_len = x.len();
    let y_len = y.len();
    let mut dp = vec![vec![None; y_len]; x_len];
    for i in 0..x_len {
        for j in 0..y_len {
            dp[i][j] = Some(
                if x[i] == y[j] {
                    (if i > 0 && j > 0 {dp[i-1][j-1].unwrap()} else {0}) + 1
                } else {
                    let x1 = if i > 0 {dp[i-1][j].unwrap()} else {0};
                    let y1 = if j > 0 {dp[i][j-1].unwrap()} else {0};
                    std::cmp::max(x1, y1)
            });
        }
    }
    dp[x_len -1][y_len -1].unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_len_test(){
        let x = "abcbdab".as_bytes();
        let y = "bdcaba".as_bytes();
        assert_eq!(max_len(&x, &y), 4);
    }
}
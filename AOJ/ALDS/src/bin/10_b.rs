fn count(m1_size: (usize, usize), m2_size: (usize, usize)) -> usize {
    debug_assert!(m1_size.1 == m2_size.0);
    m1_size.0 * m1_size.1 * m2_size.1
}

fn size_after_mul(m: &[(usize, usize)]) -> (usize, usize) {
    (m[0].0, m[m.len()-1].1)
}

// min num of scalar multi to compute M_0 * ... * M_{n-1}
fn matrix_chain_mul(rc: &[(usize, usize)]) -> usize {
    let n = rc.len();
    if n == 0 {
        panic!()
    }
    if n == 1 {
        return 0;
    }

    // dp[i][j] = min num of scalar multi to compute M_i * ... * M_{i+j}
    let mut dp: Vec<Vec<usize>> = vec![vec![]; n];

    // first case dp[i][0] = 0 for all i
    for i in 0..n {
        dp[i].push(0);
    }

    // next case dp[i][j] == min_{0 <= k < j} { minnum M_{i} * ... * M_{i+k} + minnum M_{i+k+1} * ... * M_{i+j} + r_i * c_{i+k} * c_{i+j} }
    for j in 1..n {
        for i in 0..n-j {
            let min = (0..j).map(|k| {
                dp[i][k] + dp[i+k+1][j-(k+1)]
                + count(size_after_mul(&rc[i..=i+k]), size_after_mul(&rc[i+k+1..=i+j]))
            }).min().unwrap();
            dp[i].push(min);
        }
    }

    dp[0][n-1]
}

fn main() {
    let rc = input();
    let count = matrix_chain_mul(&rc);
    println!("{}", count);
}

fn input() -> Vec<(usize, usize)> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..n).map(|i|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let v = buf.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (v[0], v[1])
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul_test() {
        let v = vec![(1,1)];
        assert_eq!(matrix_chain_mul(&v), 0);

        let v = vec![(1,1), (1,2)];
        assert_eq!(matrix_chain_mul(&v), 2);

        let v = vec![(2,3), (3,5)];
        assert_eq!(matrix_chain_mul(&v), 2 * 3 * 5);

        let v = vec![(2,3), (3,5), (5,8)];
        assert_eq!(matrix_chain_mul(&v), std::cmp::min(
            2 * 3 * 5 + 2 * 5 * 8,
            3 * 5 * 8 + 2 * 3 * 8
        ));
    }
}
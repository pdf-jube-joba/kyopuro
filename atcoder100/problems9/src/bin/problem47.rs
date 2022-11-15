fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }

}

fn maximize(a: &[usize]) -> usize {
    let n = a.len();
    let mut dp = vec![vec![None; n]; n];

    let f = |i: usize, j: usize, dp: &Vec<Vec<Option<usize>>>| {
        if i == j {
            return if n % 2 == 0 {0} else {a[i]};
        }
        if {if i < j {n - (j-i + 1)} else {i - j - 1} } % 2 == 1 {
            if a[i] < a[j] {
                dp[i][(n+j-1).rem_euclid(n)].unwrap()
            } else {
                dp[(i+1).rem_euclid(n)][j as usize].unwrap()
            }
        } else {
            let v1 = dp[(i+1).rem_euclid(n)][j].unwrap() + a[i];
            let v2 = dp[i][(n+j-1).rem_euclid(n)].unwrap() + a[j];
            std::cmp::max(v1, v2)
        }
    };

    let iter =
    (0..n).flat_map(|h|{
        (0..n).map(move |i|(i, (i+h).rem_euclid(n)))
    });

    for (i, j) in iter {
        println!("{:?} {}", (i,j), f(i, j, &dp));
        dp[i][j] = Some(f(i, j, &dp));
    }

    (0..n).filter_map(|i| dp[i][(i+n-1).rem_euclid(n)])
    .max().unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_test(){
        let a = vec![2,8,1,10,9];
        assert_eq!(maximize(&a), 18);
        let a = vec![1,10,4,5,6,2,9,3];
        assert_eq!(maximize(&a), 26);
    }
}
fn main() {
    loop {
        proconio::input! {
            n: usize,
        }
        if n != 0 {
            println!("{}", minimize(n));
        } else {break;}
    }
}

fn minimize(n: usize) -> usize {
    let n_tet = |n: usize| {n * (n+1) * (n+2) / 6};
    let mut dp = Vec::with_capacity(n+1);
    dp.push(0);
    for i in 1..=n {
        dp.push(
            (1..)
            .take_while(|j| n_tet(*j) <= i)
            .map(|j| dp[i - n_tet(j)] + 1)
            .min().unwrap()
        );
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_test(){
        assert_eq!(minimize(40), 2)
    }
}
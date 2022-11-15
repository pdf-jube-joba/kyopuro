fn main() {
    proconio::input!{
        n: usize, m: usize,
        c: [usize; m],
    }
    println!("{}", minimize(&c, n))
}

fn minimize(c: &[usize], n: usize) -> usize {
    let mut dp = vec![None; n+1];
    dp[0] = Some(0);
    for i in 0..n {
        dp[i+1] = c.iter().filter_map(|&ci|{
            if i+1 >= ci {Some(dp[i+1 - ci].unwrap() + 1)} else {None}
        }).min() ;
    }
    dp[n].unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test_1(){
        let n = 15;
        let c = vec![1,2,7,8,12,50];
        assert_eq!(minimize(&c, n), 2)
    }
    #[test]
    fn minimize_test_2(){
        let n = 65;
        let c = vec![1,2,7,8,12,50];
        assert_eq!(minimize(&c, n), 3)
    }
}
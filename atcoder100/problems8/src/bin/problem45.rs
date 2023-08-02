fn main() {
    proconio::input!{
        n: usize, m: usize,
        c: [isize; m],
        x: [u8; m],
    }

    println!("{}", minimize(&c, &x));
}

fn minimize(c: &[isize], x: &[u8]) -> usize {
    let n = x.len();
    let m = c.len();
    // dp[i][j] で i 時で j
    let mut dp: Vec<Vec<Option<usize>>> = Vec::with_capacity(n+1);
    dp.push(
        (0..=255)
        .map(|i| if i == 128 {Some(0)} else {None} )
        .collect()
    );
    for i in 1..=n {
        dp.push(
            (0..=255)
            .map(|j: usize| -> Option<usize>{
                c.iter().filter_map(|&ci| {
                    let prev = if ci.is_positive() {
                        let ci = ci as usize;
                        if j > ci && j - ci < 256 {j - ci} else {return None;}
                    } else {
                        let ci = -ci as usize;
                        if j + ci < 256 {j + ci} else {return None;}
                    };
                    dp[i-1][prev].map(|t| {
                        t + j.abs_diff(x[i-1] as usize).pow(2)
                    })
                }).min()
            }).collect()
        );
    }
    *(dp.last().unwrap().into_iter().flatten().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_test(){
        let c = vec![4,2,1,0,-1,-2,-4];
        let x = vec![131,137];
        assert_eq!(minimize(&c, &x), 2);
        let c = vec![4,2,1,0,-1,-2,-4];
        let x = vec![131,123];
        assert_eq!(minimize(&c, &x), 8);
        let c = vec![4,2,1,0,-1,-2,-4];
        let x = vec![132,134,135,134,132,128,124,122,121,122];
        assert_eq!(minimize(&c, &x), 0);
        
    }
}
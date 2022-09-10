fn main() {
    proconio::input!{
        n: usize, m: usize,
        p: [usize; n],
    }

    println!("{}", maximize(&p, m));
}

fn maximize(p: &[usize], m: usize) -> usize {
    let q: Vec<usize> = {
        let mut p = p.to_owned();
        p.push(0);
        let mut q: Vec<usize> = p.iter().flat_map(|x|{
            p.iter().map(|y|{*x + *y})
        }).collect();
        q.sort();
        q
    };
    println!("{:?}", q);
    q.iter().map(|qi|{
        let mut l = 0;
        let mut r = q.len();
        while r - l > 1 {
            let mid = (r + l) / 2;
            if *qi + q[mid] <= m {
                l = mid;
            } else {
                r = mid;
            }
        }
        if *qi + q[l] > m {0} else { *qi + q[l] }
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn maximize_test_1(){
        let p = vec![3, 14, 15, 9];
        assert_eq!(maximize(&p, 50), 48)
    }
    #[test]
    fn maximize_test_2(){
        let p = vec![16, 11, 2];
        assert_eq!(maximize(&p, 21), 20)
    }
}
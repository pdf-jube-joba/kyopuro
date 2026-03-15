fn main() {
    proconio::input! {
        n: usize,
        hs: [(usize, usize); n],
    }

    println!("{}", count(&hs));
}

fn f(hs: &[(usize, usize)], x: usize) -> bool {
    (hs.iter().all(|(h, _)|{
        x > *h
    })) && {
        let mut s: Vec<usize> = hs.iter().map(|(h, s)|{
            (x - *h) / *s
        }).collect();
        s.sort();
        s.iter().enumerate().all(|(i,s)|{
            i <= *s
        })
    }
}

fn count(hs: &[(usize, usize)]) -> usize {
    let mut l = 0; // max {l} !(f(l))
    let mut r = std::usize::MAX; // min {r} f(r)
    while r - l > 1 {
        let mid = (r + l) / 2;
        if f(hs, mid) {
            r = mid;
        } else {
            l = mid;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn f_test_1(){
        let hs = vec![(5,6), (12,4), (14,7), (21,2)];
        assert!(f(&hs, 23));
    }
    #[test]
    fn count_test_1(){
        let hs = vec![(5,6), (12,4), (14,7), (21,2)];
        assert_eq!(count(&hs), 23);
    }
    #[test]
    fn count_test_2(){
        let hs = vec![(100,1),(100,1),(100,1),(100,1),(100,1),(1,30)];
        assert_eq!(count(&hs), 105)
    }
}
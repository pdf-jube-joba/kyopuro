fn main() {
    proconio::input! {
        n: usize,
        s: [usize; n],
        q: usize,
        t: [usize; q],
    }
    
    let count = t.iter().filter(|i|{find(&s, **i)}).count();
    println!("{}" , count);

}

fn find(s: &[usize], q: usize) -> bool {
    let mut l = 0;
    let mut r = s.len();
    while r - l > 1 {
        let mid = (l + r) / 2;
        if s[mid] <= q{
            l = mid;
        } else {
            r = mid;
        }
    }
    s[l] == q
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn find_test_1(){
        let a = vec![0,1,2];
        assert!(find(&a, 0));
        assert!(find(&a, 1));
        assert!(find(&a, 2));
    }
    #[test]
    fn find_test_2(){
        let a = vec![1,3,5];
        assert!(!find(&a, 0));
        assert!(!find(&a, 2));
        assert!(!find(&a, 4));
        assert!(!find(&a, 6));
    }
}
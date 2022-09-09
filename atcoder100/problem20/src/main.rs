fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    println!("{}", count(&a, &b, &c));

}

fn count(a: &[usize], b: &[usize], c: &[usize]) -> usize {
    let mut a = a.to_owned();
    a.sort();
    let mut b = b.to_owned();
    b.sort();
    let mut c = c.to_owned();
    c.sort();
    let n = a.len();

    (0..n).map(|i|{
        let a_ind = {
            let mut l = 0; // max {l} a[l - 1] < b[i] 
            let mut r = n; // min {r} b[i] <= a[r]
            while r - l > 0 {
                let mid = (r + l) / 2;
                if b[i] <= a[mid] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            l
        }; // a[a_ind - 1] < b[i] <= a[a_ind] 
        let c_ind = {
            let mut l = 0; // max {l} c[l - 1] <= b[i]
            let mut r = n; // min {r} b[i] < c[r]
            while r - l > 0 {
                let mid = (r + l) / 2;
                if c[mid] <= b[i] {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            r
        }; // c[c_ind - 1] <= b[i] < c[c_ind] 
        a_ind * (n - c_ind)
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn count_test_1(){
        let a = vec![1, 5];
        let b = vec![2, 4];
        let c = vec![3, 6];
        assert_eq!(count(&a, &b, &c), 3)
    }
    #[test]
    fn count_test_2(){
        let a = vec![1, 1, 1];
        let b = vec![2, 2, 2];
        let c = vec![3, 3, 3];
        assert_eq!(count(&a, &b, &c), 27)
    }
    #[test]
    fn count_test_3(){
        let a = vec![3, 14, 159, 2, 6, 53];
        let b = vec![58, 9, 79, 323, 84, 6];
        let c = vec![2642, 383, 2, 79, 50, 288];
        assert_eq!(count(&a, &b, &c), 87)
    }
}
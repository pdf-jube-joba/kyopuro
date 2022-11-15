fn main() {
    proconio::input! {
        ld: usize,
        n: usize,
        m: usize,
        d: [usize; n],
        k: [usize; m],
    }

    println!("{}", sum(&d, &k, ld));

}

fn sum(d: &[usize], k: &[usize], ld: usize) -> usize {
    let mut d = d.to_owned();
    d.sort();
    d.push(ld);
    (0..k.len()).map(|i|{
        let mut l = 0;
        let mut r = d.len();
        while r - l > 1 {
            let mid = (r + l) / 2;
            if d[mid] < k[i] {
                l = mid;
            } else {
                r = mid;
            }
        }
        let l_diff = d[l].abs_diff(k[i]);
        let r_diff = d[r].abs_diff(k[i]);
        std::cmp::min(l_diff, r_diff)
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sum_test_1(){
        let ld = 8;
        let d = vec![3,1];
        let k = vec![4,6];
        assert_eq!(sum(&d, &k, ld), 3);
    }
    #[test]
    fn sum_test_2(){
        let ld = 20;
        let d = vec![12,8,16];
        let k = vec![7,7,11,8];
        assert_eq!(sum(&d, &k, ld), 3);
    }
}
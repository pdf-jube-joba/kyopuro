fn main() {   
    proconio::input! {
        n: usize,
        a: [usize; n],
        m: usize,
        q: [usize; m],
    }

    (0..m).for_each(|i|{
        println!("{}",
            if make(&a, q[i]) {"yes"} else {"no"}
        );
    });

}

fn make(a: &[usize], m: usize) -> bool {
    fn convert(n: usize, a: usize) -> Vec<bool> {
        (0..n).map(|i|{
            a & (1 << i) != 0
        }).collect()
    }
    let n = a.len();
    for bits in (0..(1 << n)).map(|i|{convert(n, i)}) {
        let sum: usize = (0..n).map(|i|{
            if bits[i] {a[i]} else {0}
        }).sum();
        if sum == m {return true;}
    }
    return false
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn make_test_1() {
        let a = vec![1, 5, 7, 10, 21];
        assert!(!make(&a, 2));
        assert!(!make(&a, 4));
        assert!(make(&a, 17));
        assert!(make(&a, 8));
    }
}
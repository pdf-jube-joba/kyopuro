fn main() {   
    proconio::input! {
        n: usize,
        a: [usize; n],
        q: usize,
        m: [usize; q],
    }

    (0..q).for_each(|i|{
        println!("{}",
            if make(&a, m[i]) {"yes"} else {"no"}
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
    (0.. (1 << n))
    .map(|i| convert(n,i))
    .any(|bits|{
        (0..n)
        .filter(|&i| bits[i])
        .map(|i|a[i])
        .sum::<usize>() == m
    })
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
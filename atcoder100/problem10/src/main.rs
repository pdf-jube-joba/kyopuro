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

fn convert(n: usize, a: i32) -> Vec<bool> {
    let mut vec = Vec::with_capacity(n);
    let mut t = a;
    for _ in 0..n {
        vec.push(t % 2 != 0);
        t = t / 2;
    }
    vec
}

fn make(a: &[usize], m: usize) -> bool {
    let n = a.len();
    for bits in (0..(1 << n)).map(|i|{convert(n, i)}) {
        let mut sum = 0;
        (0..n).for_each(|i|{
            sum += if bits[i] {a[i]} else {0};
        });
        if sum == m {return true;}
    }
    return false
}

#[test]
fn make_test_1() {
    let a = vec![1, 5, 7, 10, 21];
    assert!(!make(&a, 2));
    assert!(!make(&a, 4));
    assert!(make(&a, 17));
    assert!(make(&a, 8));
}
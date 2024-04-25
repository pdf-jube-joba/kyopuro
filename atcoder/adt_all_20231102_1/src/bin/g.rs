use num_traits::Pow;
use permutohedron::factorial;

fn main() {
    proconio::input! {
        k: usize,
    }
    println!("{}", factorial_and_multiple(k))
}

fn factorial_and_multiple(mut k: usize) -> usize {
    let mut prime_num: Vec<(usize, usize)> = vec![];
    for p in (0_usize..).take_while(move |p| p.pow(2_u32) <= k) {
        let mut n = 0;
        while k % p == 0 {
            k /= p;
            n += 1;
        }
        if n > 0 {
            prime_num.push((p, n))
        }
    }
    todo!()
}

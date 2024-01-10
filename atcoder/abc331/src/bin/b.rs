use itertools::iproduct;

fn main() {
    proconio::input! {
        mut n: usize, s: usize, m: usize, l: usize,
    }
    println!("{}", minimum(n, s, m, l))
}

fn minimum(n: usize, s: usize, m: usize, l: usize) -> usize {
    let s_num = n / 6 + 1;
    let m_num = n / 8 + 1;
    let l_num = n / 12 + 1;
    iproduct!(0..=s_num, 0..=m_num, 0..=l_num)
        .filter_map(|(i, j, k)| {
            if n <= i * 6 + j * 8 + k * 12 {
                Some(i * s + j * m + k * l)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

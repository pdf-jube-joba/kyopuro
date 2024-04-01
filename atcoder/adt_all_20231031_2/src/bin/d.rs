use itertools::iproduct;

fn main() {
    proconio::input! {
        h: usize, m: usize,
    }
    let hw: (usize, usize) = iproduct!(0..24, 0..60)
        .cycle()
        .skip_while(|&(h_, m_)| (h_, m_) != (h, m))
        .find(|&hm| swap_is_valid(hm))
        .unwrap();

    println!("{} {}", hw.0, hw.1)
}

fn swap_is_valid((h, m): (usize, usize)) -> bool {
    debug_assert!(h <= 23 && m <= 59);
    let h10 = h / 10;
    let h1 = h % 10;
    let m10 = m / 10;
    let m1 = m % 10;
    let h_ = h10 * 10 + m10;
    let m_ = h1 * 10 + m1;
    h_ <= 23 && m_ <= 59
}

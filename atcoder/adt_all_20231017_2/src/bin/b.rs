fn main() {
    proconio::input! {
        s: [usize; 8],
    }
    let is_inc_mono = s.windows(2).all(|w| {
        let [sp, sn] = w else { unreachable!() };
        sp <= sn
    });
    let is_in_range = s.iter().all(|&si| 100 <= si && si <= 675);
    let is_multiple = s.iter().all(|&si| si % 25 == 0);
    println!("{}", if is_inc_mono && is_in_range && is_multiple {"Yes"} else {"No"})
}

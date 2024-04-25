use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        r: Usize1, c: Usize1,
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if i.abs_diff(r) + j.abs_diff(c) == 1 {
                count += 1;
            }
        }
    }
    println!("{}", count)
}

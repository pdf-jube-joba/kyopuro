use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        r: Usize1, c: Usize1,
    }
    let dist = std::cmp::max(r.abs_diff(7), c.abs_diff(7));
    println!("{}", if dist % 2 == 0 {"white"} else {"black"})
}

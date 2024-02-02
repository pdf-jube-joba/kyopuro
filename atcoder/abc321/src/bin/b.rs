fn main() {
    proconio::input! {
        n: usize, x: usize,
        mut a: [usize; n-1],
    }
    a.sort();
    let now_sum: usize = a.iter().sum();
    let now_lowest = a.first().unwrap();
    let now_highest = a.last().unwrap();
    if now_sum - now_highest >= x {
        println!("0");
    } else if now_sum - now_lowest < x {
        println!("-1");
    } else {
        let shortage = x - (now_sum - now_highest - now_lowest);
        println!("{}", shortage);
    }
}

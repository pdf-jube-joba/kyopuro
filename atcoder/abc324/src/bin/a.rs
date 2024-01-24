fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }

    let b = a.iter().all(|ai| *ai == a[0]);
    println!("{}", if b {"Yes"} else {"No"})
}

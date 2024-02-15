fn main() {
    proconio::input! {
        n: usize, x: usize,
        p: [usize; n],
    }
    let pos = p.into_iter().position(|pk| pk == x).unwrap();
    println!("{}", pos + 1)
}

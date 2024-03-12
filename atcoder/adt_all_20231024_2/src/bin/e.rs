use itertools::iproduct;

const L: usize = 7;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        b: [[usize; m]; n],
    }
    println!("{}", if is_sub(b) { "Yes" } else { "No" })
}

fn is_sub(b: Vec<Vec<usize>>) -> bool {
    let (n, m) = (b.len(), b[0].len());
    let ij = |a: usize| -> (usize, usize) { (a / L, a % L) };
    let (first_i, first_j) = ij(b[0][0] - 1);
    iproduct!(0..n, 0..m).all(|(i, j)| {
        let (i0, j0) = (first_i + i, first_j + j);
        // eprintln!("{i} {j} {i0} {j0}");
        if j0 >= 7 {
            return false;
        }
        b[i][j] == i0 * 7 + j0 + 1
    })
}

use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
    }
    let at = transpose(a);
    for ati in at {
        println!("{}", ati.into_iter().join(" "))
    }
}

fn transpose(a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let (h, w) = (a.len(), a[0].len());
    let mut at = vec![vec![0; h]; w];
    for i in 0..w {
        for j in 0..h {
            at[i][j] = a[j][i];
        }
    }
    at
}

fn main() {
    proconio::input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut is_covered: Vec<Vec<bool>> = vec![vec![false; 101]; 101];

    for (a, b, c, d) in abcd {
        for i in a..b {
            for j in c..d {
                is_covered[i][j] = true;
            }
        }
    }
    let res = is_covered
        .into_iter()
        .map(|v| v.into_iter())
        .flatten()
        .filter(|b| *b)
        .count();

    println!("{res}")
}

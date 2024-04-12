use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        a: [Chars; n],
    }
    println!("{}", if tournament_result(a) { "correct" } else { "incorrect" })
}

fn tournament_result(a: Vec<Vec<char>>) -> bool {
    let n = a.len();
    for i in 0..n {
        for j in i + 1..n {
            if !matches!((a[i][j], a[j][i]), ('W', 'L') | ('L', 'W') | ('D', 'D')) {
                return false;
            }
        }
    }
    true
}

use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: [Chars; 10]
    }
    let (a, b, c, d) = find_rectangle(s);
    println!("{} {}\n{} {}", a + 1, b + 1, c + 1, d + 1);
}

fn find_rectangle(s: Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let (h, w) = (s.len(), s[0].len());

    let (a, b) = (0..w)
        .find_map(|j| {
            let l = (0..h).position(|i| s[i][j] == '#')?;
            let r = (0..h).rposition(|i| s[i][j] == '#')?;
            Some((l, r))
        })
        .unwrap();

    let (c, d) = (0..h)
        .find_map(|i| {
            let l = (0..w).position(|j| s[i][j] == '#')?;
            let r = (0..w).rposition(|j| s[i][j] == '#')?;
            Some((l, r))
        })
        .unwrap();

    (a, b, c, d)
}

use std::collections::VecDeque;

fn input() -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    proconio::input! {
        h: usize, w: usize, n: usize,
        input1: [[char; w]; h],
    }
    let mut f = vec![(0, 0); n + 1];
    let mut m = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            match input1[i][j] {
                '.' => {
                    m[i][j] = true;
                }
                'X' => {
                    m[i][j] = false;
                }
                c => {
                    if let Some(l) = c.to_digit(10) {
                        let num: usize = l.try_into().unwrap();
                        f[num + 1] = (i, j);
                        m[i][j] = true;
                    } else {
                        f[0] = (i, j);
                        m[i][j] = true;
                    }
                }
            }
        }
    }
    (m, f)
}

fn main() {
    let (m, f) = input();
    let total: usize = (0..f.len()).map(|i| path_length(&m, f[i], f[i + 1])).sum();
    println!("{}", total);
}

fn path_length(m: &[Vec<bool>], s: (usize, usize), t: (usize, usize)) -> usize {
    let h = m.len();
    let w = m[0].len();
    let mut memo = vec![vec![None; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back(s);
    memo[s.0][s.1] = Some(0);
    while let Some(now) = queue.pop_front() {
        let l = memo[now.0][now.1].unwrap();
        vec![
            if 0 < now.0 {
                Some((now.0 - 1, now.1))
            } else {
                None
            },
            if now.0 < h - 1 {
                Some((now.0 + 1, now.1))
            } else {
                None
            },
            if 0 < now.1 {
                Some((now.0, now.1 - 1))
            } else {
                None
            },
            if now.1 < w - 1 {
                Some((now.0, now.1 + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .for_each(|(i, j)| {
            if memo[i][j].is_none() {
                queue.push_back((i, j));
                memo[i][j] = Some(l + 1);
            }
        });
    }
    memo[t.0][t.1].unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_length_test() {
        let m = vec![vec![true; 3]; 3];
        let s = (0, 0);
        let t = (2, 2);
        let result = path_length(&m, s, t);
        assert_eq!(result, 4);
    }
}

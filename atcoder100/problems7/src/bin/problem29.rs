use std::collections::VecDeque;

fn main() {
    proconio::input! {
        r: usize, c: usize,
        s: (usize, usize),
        g: (usize, usize),
        c: [[char; c]; r],
    }
    let c: Vec<Vec<bool>> = c
        .iter()
        .map(|ci| {
            ci.iter()
                .map(|&char| match char {
                    '.' => true,
                    '#' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
}

fn search(v: &[Vec<bool>], s: (usize, usize), t: (usize, usize)) -> usize {
    let r = v.len();
    let c = v[0].len();
    let mut result = vec![vec![None; c]; r];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(s);
    result[s.0][s.1] = Some(0);
    while let Some(now) = queue.pop_front() {
        let len = result[now.0][now.1].unwrap();
        let nexts: Vec<(usize, usize)> = vec![
            if 0 < now.0 {
                Some((now.0 - 1, now.1))
            } else {
                None
            },
            if now.0 < r - 1 {
                Some((now.0 + 1, now.1))
            } else {
                None
            },
            if 0 < now.1 {
                Some((now.0, now.1 - 1))
            } else {
                None
            },
            if now.1 < c - 1 {
                Some((now.0, now.1 + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();
        nexts.iter().for_each(|&next| {
            if v[next.0][next.1] && result[next.0][next.1].is_none() {
                result[next.0][next.1] = Some(len + 1);
                queue.push_back(next);
            }
        });
    }
    result[t.0][t.1].unwrap()
}

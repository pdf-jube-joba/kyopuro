use std::collections::VecDeque;

fn input() -> Vec<Vec<bool>> {
    proconio::input! {
        h: usize, w: usize,
        s: [[char; w]; h],
    }
    s.iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match *c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let wall = input();
    let score = score_max(&wall);
    println!("{}", score);
}

fn score_max(wall: &[Vec<bool>]) -> usize {
    let h = wall.len();
    let w = wall[0].len();
    let surr = |pos: (usize, usize)| {
        vec![
            if 0 < pos.0 && !wall[pos.0 - 1][pos.1] {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            },
            if pos.0 < h - 1 && !wall[pos.0 + 1][pos.1] {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            },
            if 0 < pos.1 && !wall[pos.0][pos.1 - 1] {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            },
            if pos.1 < w - 1 && !wall[pos.0][pos.1 + 1] {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
    };

    let mut check = vec![vec![None; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    check[0][0] = Some(1);

    while let Some(now) = queue.pop_front() {
        let len = check[now.0][now.1].unwrap();
        for next in surr(now) {
            if check[next.0][next.1].is_none() {
                check[next.0][next.1] = Some(len + 1);
                queue.push_back(next);
            }
        }
    }

    let all: usize = wall.iter().map(|v| v.iter().filter(|b| !**b).count()).sum();
    let path = check[h - 1][w - 1].unwrap();
    all - path
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_max_test_1() {
        let wall = vec![
            vec![false, false, true],
            vec![true, false, false],
            vec![false, false, false],
        ];
        let result = score_max(&wall);
        assert_eq!(result, 2);
    }
}

use std::collections::VecDeque;

fn to_bool(vec: &[usize]) -> Vec<bool> {
    (0..vec.len())
        .map(|i| match vec[i] {
            0 => false,
            1 => true,
            _ => unreachable!(),
        })
        .collect()
}

fn input() -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    proconio::input! {
        h: usize, w: usize,
    }
    let mut w_wall: Vec<Vec<bool>> = vec![Vec::new(); h];
    let mut h_wall: Vec<Vec<bool>> = vec![Vec::new(); h - 1];
    proconio::input! {
        w1: [usize; w],
    }
    h_wall[0] = to_bool(&w1);
    for i in 0..h - 1 {
        proconio::input! {
            w2: [usize; w-1],
            w3: [usize; w],
        }
        w_wall[i + 1] = to_bool(&w2);
        h_wall[i] = to_bool(&w3);
    }
    (w_wall, h_wall)
}

fn main() {
    let (w_wall, h_wall) = input();
    let result = min_length(&w_wall, &h_wall);
    println!("{}", result);
}

fn min_length(w_wall: &[Vec<bool>], h_wall: &[Vec<bool>]) -> usize {
    let h = w_wall.len();
    let w = w_wall[0].len() + 1;

    let surr = |(y, x): (usize, usize)| {
        vec![
            if 0 < y && !h_wall[y - 1][x] {
                Some((y - 1, x))
            } else {
                None
            }, //up
            if y < h - 1 && !h_wall[y][x] {
                Some((y + 1, x))
            } else {
                None
            }, //down
            if 0 < x && !w_wall[y][x - 1] {
                Some((y, x - 1))
            } else {
                None
            }, //left
            if x < w - 1 && !w_wall[y][x] {
                Some((y, x + 1))
            } else {
                None
            }, //right
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

    match check[h - 1][w - 1] {
        Some(len) => len,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    #[test]
    fn min_length_test_1() {
        let w_wall = vec![vec![1], vec![0], vec![1]]
            .into_iter()
            .map(|vec| to_bool(&vec))
            .collect::<Vec<Vec<bool>>>();
        let h_wall = vec![vec![0, 1], vec![1, 0]]
            .into_iter()
            .map(|vec| to_bool(&vec))
            .collect::<Vec<Vec<bool>>>();
        let result = min_length(&w_wall, &h_wall);
        assert_eq!(result, 4);
    }
    #[test]
    fn min_length_test_2() {
        let w_wall = vec![
            vec![1, 0, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 1, 0],
        ]
        .into_iter()
        .map(|vec| to_bool(&vec))
        .collect::<Vec<Vec<bool>>>();
        let h_wall = vec![
            vec![0, 1, 1, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1, 1, 0, 0, 0],
        ]
        .into_iter()
        .map(|vec| to_bool(&vec))
        .collect::<Vec<Vec<bool>>>();
        let result = min_length(&w_wall, &h_wall);
        assert_eq!(result, 0);
    }
}

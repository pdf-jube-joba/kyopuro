fn adjacent(n: usize) -> Vec<usize> {
    vec![
        // left
        if n == 0 || n == 3 || n == 6 {
            None
        } else {
            Some(n - 1)
        },
        // up
        if n == 0 || n == 1 || n == 2 {
            None
        } else {
            Some(n - 3)
        },
        // right
        if n == 2 || n == 5 || n == 8 {
            None
        } else {
            Some(n + 1)
        },
        // down
        if n == 6 || n == 7 || n == 8 {
            None
        } else {
            Some(n + 3)
        },
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect()
}

fn is_solved(puzzle: &[usize]) -> bool {
    (0..9).all(|i| (i + 1) % 9 == puzzle[i])
}

fn next_moves(puzzle: &[usize]) -> Vec<Vec<usize>> {
    let i = (0..9).position(|i| puzzle[i] == 0).unwrap();
    let mut v = vec![];
    for j in adjacent(i) {
        let mut v2 = puzzle.to_vec();
        v2.swap(i, j);
        v.push(v2);
    }
    v
}

fn index(puzzle: &[usize]) -> usize {
    use std::convert::TryInto;
    (0..9)
        .map(|i| puzzle[i] * (9_usize.pow(i.try_into().unwrap())))
        .sum()
}

fn puzzle_solve_min(puzzle: Vec<usize>) -> usize {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut visited = vec![false; index(&[0, 1, 2, 3, 4, 5, 6, 7, 8])];

    visited[index(&puzzle)] = true;
    queue.push_back((puzzle, 0));

    while let Some((puzzle, moves)) = queue.pop_front() {
        if is_solved(&puzzle) {
            return moves;
        }
        for next_move in next_moves(&puzzle) {
            if !visited[index(&next_move)] {
                visited[index(&next_move)] = true;
                queue.push_back((next_move, moves + 1));
            }
        }
    }
    unreachable!()
}

fn main() {
    let puzzle = input();
    let moves = puzzle_solve_min(puzzle);
    println!("{}", moves);
}

fn input() -> Vec<usize> {
    let mut v = Vec::with_capacity(9);

    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .for_each(|i| {
            v.push(i);
        });

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .for_each(|i| {
            v.push(i);
        });

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .for_each(|i| {
            v.push(i);
        });

    v
}
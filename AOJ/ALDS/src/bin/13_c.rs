const LEN: usize = 4;
const LEN2: usize = 16; //LEN.pow(2);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    U,
    R,
    D,
    L,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Puzzle {
    puzzle: [u8; LEN2],
}

const SOLVED_PUZZLE: Puzzle = Puzzle {
    puzzle: [
         1, 2, 3, 4,
         5, 6, 7, 8,
         9,10,11,12,
        13,14,15, 0,
    ],
};

impl Puzzle {
    fn from_vec(slice: &[u8]) -> Self {
        debug_assert!(slice.len() == LEN2);
        let mut puzzle = Self { puzzle: [0; LEN2] };
        for i in 0..LEN2 {
            puzzle.puzzle[i] = slice[i];
        }
        puzzle
    }
    fn move_0(&mut self, m: Move) -> bool {
        let pos_0 = (0..LEN2).position(|i| self.puzzle[i] == 0).unwrap();
        match m {
            Move::U if LEN <= pos_0 => {
                self.puzzle.swap(pos_0, pos_0 - LEN);
            }
            Move::R if pos_0 % LEN != LEN - 1 => {
                self.puzzle.swap(pos_0, pos_0 + 1);
            }
            Move::D if pos_0 < LEN2 - LEN => {
                self.puzzle.swap(pos_0, pos_0 + LEN);
            }
            Move::L if pos_0 % LEN != 0 => {
                self.puzzle.swap(pos_0, pos_0 - 1);
            }
            _ => {
                return false;
            }
        }
        true
    }
}

fn is_solved(puzzle: &Puzzle) -> bool {
    *puzzle == SOLVED_PUZZLE
}

fn next_moves(puzzle: &Puzzle) -> Vec<Puzzle> {
    let mut next_puzzles = Vec::with_capacity(4);
    for m in vec![Move::U, Move::R, Move::D, Move::L] {
        let mut next_puzzle = puzzle.clone();
        if next_puzzle.move_0(m) {
            next_puzzles.push(next_puzzle);
        }
    }
    next_puzzles
}

fn puzzle_solve_min(puzzle: Puzzle) -> usize {
    use std::collections::BTreeSet;
    let moves = [Move::U, Move::R, Move::D, Move::L];

    let mut s_pre: BTreeSet<Puzzle> = BTreeSet::new();
    let mut s_now: BTreeSet<Puzzle> = BTreeSet::new();
    s_now.insert(puzzle);
    let mut s_nex: BTreeSet<Puzzle> = BTreeSet::new();

    let mut depth = 0;

    loop {
        for now_puzzle in s_now.iter() {
            if is_solved(&now_puzzle) {
                return depth;
            }
            for m in &moves {
                let mut next_puzzle = now_puzzle.clone();
                if next_puzzle.move_0(*m) && !s_pre.contains(&next_puzzle) {
                    s_nex.insert(next_puzzle);
                }
            };
        }
        depth += 1;
        s_pre = s_now;
        s_now = s_nex;
        s_nex = BTreeSet::new();
    }
}

// fn puzzle_solve_min(puzzle: Puzzle) -> usize {
//     use std::collections::VecDeque;
//     let mut queue = VecDeque::new();

//     queue.push_back((puzzle, 0));

//     while let Some((puzzle, moves)) = queue.pop_front() {
//         if is_solved(&puzzle) {
//             return moves;
//         }
//         for next_move in next_moves(&puzzle) {
//             queue.push_back((next_move, moves + 1));
//         }
//     }
//     unreachable!()
// }

fn main() {
    let puzzle = input();
    let moves = puzzle_solve_min(puzzle);
    println!("{}", moves);
}

fn input() -> Puzzle {
    let mut v = Vec::with_capacity(LEN2);

    let mut buf = String::new();
    let stdin = std::io::stdin();

    for _ in 0..LEN {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|str| str.parse::<u8>().unwrap())
            .for_each(|i| {
                v.push(i);
            });
    }
    Puzzle::from_vec(&v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_test() {
        let puzzle = vec![
             1, 2, 3, 4,
             6, 7, 8, 0,
             5,10,11,12,
             9,13,14,15,
            ];
        let min = puzzle_solve_min(Puzzle::from_vec(&puzzle));
        assert_eq!(min, 8);
    }
}

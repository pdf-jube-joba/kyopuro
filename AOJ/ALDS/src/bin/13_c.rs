const LEN: usize = 4;
const LEN2: usize = 16; //LEN.pow(2);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    U,
    R,
    D,
    L,
}

impl Move {
    fn neg(self) -> Self {
        match self {
            Move::U => Move::D,
            Move::R => Move::L,
            Move::D => Move::U,
            Move::L => Move::R,
        }
    }
}

const MOVES: [Move; 4] = [Move::U, Move::R, Move::D, Move::L];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    puzzle: [u8; LEN2],
    pos: usize,
}

impl Puzzle {
    fn from_vec(slice: &[u8]) -> Self {
        debug_assert!(slice.len() == LEN.pow(2));
        let mut puzzle = Self {
            puzzle: [0; LEN2],
            pos: 0,
        };
        for i in 0..LEN2 {
            puzzle.puzzle[i] = slice[i];
            if slice[i] == 0 {
                puzzle.pos = i;
            }
        }
        puzzle
    }
    fn move_0(&mut self, m: Move) -> bool {
        match m {
            Move::U if self.pos >= LEN => {
                self.puzzle.swap(self.pos, self.pos - LEN);
                self.pos -= LEN;
            }
            Move::R if self.pos % LEN != LEN - 1 => {
                self.puzzle.swap(self.pos, self.pos + 1);
                self.pos += 1;
            }
            Move::D if self.pos + LEN < LEN2 => {
                self.puzzle.swap(self.pos, self.pos + LEN);
                self.pos += LEN;
            }
            Move::L if self.pos % LEN != 0 => {
                self.puzzle.swap(self.pos, self.pos - 1);
                self.pos -= 1;
            }
            _ => {
                return false;
            }
        }
        true
    }
    fn manhattan_dist_from_solved(&self) -> usize {
        let mut sum = 0;
        for i in 0..LEN2 {
            let mut v = self.puzzle[i] as usize;
            if v != 0 {
                v -= 1;
                sum += abs(v / LEN, i / LEN) + abs(v % LEN, i % LEN);
            }
        }
        sum
    }
}

fn abs(u1: usize, u2: usize) -> usize {
    if u1 < u2 {
        u2 - u1
    } else {
        u1 - u2
    }
}

// if dfs returns false then puzzle is change nothing
fn dfs(puzzle: &mut Puzzle, pred_move: Option<Move>, depth: usize, limit: usize) -> bool {
    let minimal_dist = puzzle.manhattan_dist_from_solved();
    if minimal_dist + depth > limit {
        return false;
    }
    if minimal_dist == 0 {
        // <=> puzzle == SOLVED_PUZZLE
        return true;
    }
    for &m in &MOVES {
        if Some(m.neg()) == pred_move {
            continue;
        }
        if puzzle.move_0(m) {
            if dfs(puzzle, Some(m), depth + 1, limit) {
                return true;
            }
            puzzle.move_0(m.neg());
        }
    }
    false
}

fn puzzle_solve_dfs_with_limit(mut puzzle: Puzzle) -> usize {
    // 45 comes from constraint
    for i in 0..=45 {
        if dfs(&mut puzzle, None, 0, i) {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let puzzle = input();
    let moves = puzzle_solve_dfs_with_limit(puzzle);
    println!("{}", moves);
}

fn input() -> Puzzle {
    let mut v = Vec::with_capacity(LEN.pow(2));

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
    fn move_test() {
        let puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let mut puzzle = Puzzle::from_vec(&puzzle);
        assert!(!puzzle.move_0(Move::R));
        assert!(!puzzle.move_0(Move::D));
        for m in vec![Move::L, Move::U, Move::R, Move::D] {
            assert!(puzzle.move_0(m));
        }
        let expect = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 13, 14, 11, 0];
        let expect = Puzzle::from_vec(&expect);
        assert_eq!(puzzle, expect);
    }
    #[test]
    fn min_dist_test() {
        let puzzle = Puzzle::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]);
        assert_eq!(puzzle.manhattan_dist_from_solved(), 0);

        let puzzle = Puzzle::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15]);
        assert_eq!(puzzle.manhattan_dist_from_solved(), 1);

        let puzzle = Puzzle::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 14, 15]);
        assert_eq!(puzzle.manhattan_dist_from_solved(), 2);
    }
    #[test]
    fn min_test() {
        let puzzle = vec![1, 2, 3, 4, 6, 7, 8, 0, 5, 10, 11, 12, 9, 13, 14, 15];
        let mut puzzle = Puzzle::from_vec(&puzzle);
        let min = puzzle_solve_dfs_with_limit(puzzle);
        assert_eq!(min, 8);
    }
    #[test]
    fn tle_test() {
        let puzzle = vec![14, 7, 6, 4, 2, 3, 1, 11, 5, 9, 12, 15, 13, 0, 10, 8];
        let mut puzzle = Puzzle::from_vec(&puzzle);
        let min = puzzle_solve_dfs_with_limit(puzzle);
        assert_eq!(min, 40)
    }
}

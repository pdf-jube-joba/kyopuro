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
    puzzle: [[u8; LEN]; LEN],
}

impl Puzzle {
    fn from_vec(slice: &[u8]) -> Self {
        debug_assert!(slice.len() == LEN.pow(2));
        let mut puzzle = Self {
            puzzle: [[0; LEN]; LEN],
        };
        for i in 0..LEN {
            for j in 0..LEN {
                puzzle.puzzle[i][j] = slice[i * LEN + j];
            }
        }
        puzzle
    }
    fn move_0(&mut self, m: Move) -> bool {
        let (pos0_x, pos0_y) = {
            (0..LEN)
                .find_map(|i| {
                    (0..LEN)
                        .position(|j| self.puzzle[i][j] == 0)
                        .map(|j| (i, j))
                })
                .unwrap()
        };
        match m {
            Move::U if 0 < pos0_x => {
                self.puzzle[pos0_x][pos0_y] = self.puzzle[pos0_x - 1][pos0_y];
                self.puzzle[pos0_x - 1][pos0_y] = 0;
            }
            Move::R if pos0_y < LEN - 1 => {
                self.puzzle[pos0_x][pos0_y] = self.puzzle[pos0_x][pos0_y + 1];
                self.puzzle[pos0_x][pos0_y + 1] = 0;
            }
            Move::D if pos0_x < LEN - 1 => {
                self.puzzle[pos0_x][pos0_y] = self.puzzle[pos0_x + 1][pos0_y];
                self.puzzle[pos0_x + 1][pos0_y] = 0;
            }
            Move::L if 0 < pos0_y => {
                self.puzzle[pos0_x][pos0_y] = self.puzzle[pos0_x][pos0_y - 1];
                self.puzzle[pos0_x][pos0_y - 1] = 0;
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
            let mut v = self.puzzle[i / LEN][i % LEN] as usize;
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
        // let puzzle = vec![14, 7, 6, 4, 2, 3, 1, 11, 5, 9, 12, 15, 13, 0, 10, 8];
        // let mut puzzle = Puzzle::from_vec(&puzzle);
        // let min = puzzle_solve_dfs_with_limit(puzzle);
        // assert_eq!(min, 40)
    }
}

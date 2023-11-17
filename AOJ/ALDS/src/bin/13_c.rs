const LEN: usize = 4;
const LEN2: usize = 16; //LEN.pow(2);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    U,
    R,
    D,
    L,
}

const MOVES: [Move; 4] = [Move::U, Move::R, Move::D, Move::L];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    puzzle: [u8; LEN2],
}

const SOLVED_PUZZLE: Puzzle = Puzzle {
    puzzle: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
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
    fn dist(&self, other: &Self) -> usize {
        (0..LEN2)
            .map(|i| {
                if self.puzzle[i] == other.puzzle[i] {
                    0
                } else {
                    1
                }
            })
            .sum()
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

// we want priorityqueue
// std::collections::BinaryHeap needs T: Ord so we can't use it
#[derive(Debug, Clone, PartialEq, Eq)]
struct PriorityQueue<T, F: Fn(&T) -> usize> {
    vec: Vec<T>,
    func: F,
}

fn parent(i: usize) -> Option<usize> {
    if i > 0 {
        Some((i - 1) / 2)
    } else {
        None
    }
}

fn right(i: usize) -> usize {
    2 * i + 1
}

fn left(i: usize) -> usize {
    2 * i + 2
}

impl<T, F: Fn(&T) -> usize> PriorityQueue<T, F> {
    fn new(f: F) -> Self {
        Self {
            vec: vec![],
            func: f,
        }
    }
    fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }
    fn push(&mut self, value: T) {
        self.vec.push(value);

        let mut i = self.vec.len() - 1;

        while let Some(p) = parent(i) {
            if (self.func)(&self.vec[i]) > (self.func)(&self.vec[p]) {
                return;
            }
            self.vec.swap(i, p);
            i = p;
        }
    }
    fn pop(&mut self) -> Option<T> {
        let n = self.vec.len();
        if n == 0 || n == 1 {
            self.vec.pop()
        } else {
            self.vec.swap(0, n - 1);
            let value = self.vec.pop();
            self.min_heapify(0);
            value
        }
    }
    fn min_heapify(&mut self, i: usize) {
        let l = left(i);
        let r = right(i);

        let prio_i: Option<(usize, usize)> = self.get(i).map(|val| ((self.func)(val), i));
        let prio_l: Option<(usize, usize)> = self.get(l).map(|val| ((self.func)(val), l));
        let prio_r: Option<(usize, usize)> = self.get(r).map(|val| ((self.func)(val), r));

        let m: usize = {
            vec![prio_i, prio_l, prio_r]
                .into_iter()
                .filter_map(|v| v)
                .min_by_key(|v| v.0)
                .unwrap()
                .1
        };

        if m != i {
            self.vec.swap(i, m);
            self.min_heapify(m);
        }
    }
}

fn key_compute(&(ref puzzle, depth): &(Puzzle, usize)) -> usize {
    puzzle.dist(&SOLVED_PUZZLE)
}

fn puzzle_solve_min(puzzle: Puzzle) -> usize {
    use std::collections::{BinaryHeap, HashSet};

    let mut is_visited: HashSet<Puzzle> = HashSet::new();

    // puzzle depth
    let mut queue: PriorityQueue<(Puzzle, usize), _> = PriorityQueue::new(key_compute);
    queue.push((puzzle, 0));

    while let Some((puzzle, depth)) = queue.pop() {
        if is_solved(&puzzle) {
            return depth;
        }
        is_visited.insert(puzzle.clone());
        for m in &MOVES {
            let mut next_puzzle = puzzle.clone();
            if next_puzzle.move_0(*m) && !is_visited.contains(&next_puzzle) {
                queue.push((next_puzzle, depth + 1));
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
    fn min_test_usize(val: &(usize, usize)) -> usize {
        val.0
    }
    #[test]
    fn minheap_test() {
        let mut heap = PriorityQueue::new(min_test_usize);
        heap.push((0, 0));
        assert_eq!(heap.pop(), Some((0, 0)));
        assert_eq!(heap.pop(), None);

        heap.push((1, 0));
        assert_eq!(heap.pop(), Some((1, 0)));
        assert_eq!(heap.pop(), None);

        heap.push((0, 0));
        heap.push((1, 0));
        assert_eq!(heap.pop(), Some((0, 0)));
        assert_eq!(heap.pop(), Some((1, 0)));
        assert_eq!(heap.pop(), None);

        heap.push((1, 0));
        heap.push((0, 0));
        assert_eq!(heap.pop(), Some((0, 0)));
        assert_eq!(heap.pop(), Some((1, 0)));
        assert_eq!(heap.pop(), None);
    }
    use super::*;
    #[test]
    fn min_test() {
        let puzzle = vec![1, 2, 3, 4, 6, 7, 8, 0, 5, 10, 11, 12, 9, 13, 14, 15];
        let min = puzzle_solve_min(Puzzle::from_vec(&puzzle));
        assert_eq!(min, 8);
    }
}

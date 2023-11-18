fn main() {
    time();
}

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

struct A {
    b: [[u8; LEN]; LEN],
}

const SOLVED: A = A {
    b: [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]],
};

impl A {
    fn move_0(&mut self, m: Move) -> bool {
        let (pos0_x, pos0_y) = {
            (0..LEN)
                .find_map(|i| (0..LEN).position(|j| self.b[i][j] == 0).map(|j| (i, j)))
                .unwrap()
        };
        match m {
            Move::U if 0 < pos0_x => {
                self.b[pos0_x][pos0_y] = self.b[pos0_x - 1][pos0_y];
                self.b[pos0_x - 1][pos0_y] = 0;
            }
            Move::R if pos0_y < LEN - 1 => {
                self.b[pos0_x][pos0_y] = self.b[pos0_x][pos0_y + 1];
                self.b[pos0_x][pos0_y + 1] = 0;
            }
            Move::D if pos0_x < LEN - 1 => {
                self.b[pos0_x][pos0_y] = self.b[pos0_x + 1][pos0_y];
                self.b[pos0_x + 1][pos0_y] = 0;
            }
            Move::L if 0 < pos0_y => {
                self.b[pos0_x][pos0_y] = self.b[pos0_x][pos0_y - 1];
                self.b[pos0_x][pos0_y - 1] = 0;
            }
            _ => {
                return false;
            }
        }
        true
    }
    fn dist(&self, other: &Self) -> usize {
        let mut sum = 0;
        for i in 0..LEN {
            for j in 0..LEN {
                if self.b[i][j] != 0 {
                    sum += if self.b[i][j] == other.b[i][j] { 0 } else { 1 };
                }
            }
        }
        sum
    }
}

struct B {
    b: [u8; LEN2],
    p: usize,
}

impl B {
    fn move_0(&mut self, m: Move) -> bool {
        match m {
            Move::U if self.p < LEN => {
                self.b.swap(self.p, self.p - LEN);
                self.p -= LEN;
            }
            Move::R if self.p % LEN == LEN - 1 => {
                self.b.swap(self.p, self.p + 1);
                self.p += 1;
            }
            Move::D if self.p + LEN < LEN2 - 1 => {
                self.b.swap(self.p, self.p + LEN);
                self.p += LEN;
            }
            Move::U if self.p % LEN == 0 => {
                self.b.swap(self.p, self.p - 1);
                self.p -= 1;
            }
            _ => {
                return false;
            }
        }
        true
    }
    fn get_md(&self) -> usize {
        let mut sum = 0;
        for i in 0..16 {
            let v = self.b[i] as usize;
            if v != 0 {
                sum += (v / 4).abs_diff(i / 4) + (v % 4).abs_diff(i % 4);
            }
        }
        sum
    }
}

// 64 bit = 4 bit * 16
// b の 4*i + 0,...,4*i+3 が i を表す
struct C {
    b: usize,
    p: usize,
}

// move　は 0,1,2,3 で URDL とする

fn swap(u: usize, mut i: usize, mut j: usize) -> usize {
    if j < i {
        std::mem::swap(&mut i, &mut j);
    }
    // i <= j
    (u & (15 << (4 * i))) << (4 * (j - i))
        | (u & (15 << (4 * j))) >> (4 * (j - i))
        | u & !(15 << (4 * i)) & !(15 << (4 * j))
}

impl C {
    fn from_slice(slice: &[u8; 16]) -> Self {
        let mut b = 0;
        let mut p = 0;
        for i in 0..16 {
            b += (slice[i] as usize) << (4 * i);
            if slice[i] == 0 {
                p = i;
            }
        }
        Self { b, p }
    }
    fn move_0(&mut self, m: usize) -> bool {
        match m {
            // U
            0 if 4 <= self.p => {
                self.b = swap(self.b, self.p, self.p - 4);
            }
            1 if self.p % 4 != 3 => {
                self.b = swap(self.b, self.p, self.p + 1);
            }
            2 if self.p < 11 => {
                self.b = swap(self.b, self.p, self.p + 4);
            }
            3 if self.p % 4 != 0 => {
                self.b = swap(self.p, self.p, self.p - 1);
            }
            _ => {
                return false;
            }
        }
        true
    }
}

pub fn time() {
    let mut a = A {
        b: [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]],
    };

    let start_time = std::time::Instant::now();
    for _ in 0..10_000_000 {
        a.move_0(Move::R);
        a.move_0(Move::L);
    }
    let end_time = std::time::Instant::now();
    eprintln!("a {:?}", end_time - start_time);

    let mut b = B {
        b: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        p: 0,
    };

    let start_time = std::time::Instant::now();
    for _ in 0..10_000_000 {
        b.move_0(Move::R);
        b.move_0(Move::L);
    }
    let end_time = std::time::Instant::now();
    println!("b {:?}", end_time - start_time);

    let mut c = C::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let start_time = std::time::Instant::now();
    for _ in 0..10_000_000 {
        c.move_0(1);
        c.move_0(3);
    }
    let end_time = std::time::Instant::now();
    println!("c {:?}", end_time - start_time);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bits_test() {
        assert_eq!(swap(0, 0, 0), 0);
        assert_eq!(swap(15, 0, 0), 15);
        assert_eq!(swap(15, 0, 1), 15 << 4);
        assert_eq!(swap(15 << 4, 0, 1), 15);
        assert_eq!(swap(15 << 4, 5, 3), 15 << 4);
        assert_eq!(
            swap((15 << 4 * 1) + (15 << 4 * 3), 3, 5),
            (15 << 4 * 1) + (15 << 4 * 5)
        );
    }
}

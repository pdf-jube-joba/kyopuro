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
}

fn main() {
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
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiceRoll {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
}

impl DiceRoll {
    fn rev(&self) -> DiceRoll {
        match self {
            DiceRoll::R1 => Self::R6,
            DiceRoll::R2 => Self::R5,
            DiceRoll::R3 => Self::R4,
            DiceRoll::R4 => Self::R3,
            DiceRoll::R5 => Self::R2,
            DiceRoll::R6 => Self::R1,
        }
    }
    fn right_hand_around(&self) -> Vec<DiceRoll> {
        match self {
            DiceRoll::R1 => vec![DiceRoll::R4, DiceRoll::R2, DiceRoll::R3, DiceRoll::R5],
            DiceRoll::R2 => vec![DiceRoll::R1, DiceRoll::R4, DiceRoll::R6, DiceRoll::R3],
            DiceRoll::R3 => vec![DiceRoll::R1, DiceRoll::R2, DiceRoll::R6, DiceRoll::R5],
            DiceRoll::R4 => vec![DiceRoll::R1, DiceRoll::R5, DiceRoll::R6, DiceRoll::R2],
            DiceRoll::R5 => vec![DiceRoll::R1, DiceRoll::R3, DiceRoll::R6, DiceRoll::R4],
            DiceRoll::R6 => vec![DiceRoll::R2, DiceRoll::R4, DiceRoll::R5, DiceRoll::R3],
            _ => unreachable!(),
        }
    }
}

struct Location {
    up: DiceRoll,
    south: DiceRoll,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            up: DiceRoll::R1,
            south: DiceRoll::R2,
        }
    }
}

impl Location {
    fn up(&self) -> DiceRoll {
        self.up
    }
    fn down(&self) -> DiceRoll {
        self.up().rev()
    }
    fn south(&self) -> DiceRoll {
        self.south
    }
    fn north(&self) -> DiceRoll {
        self.south().rev()
    }
    
    fn spot_by_index(&self, front: DiceRoll, index: usize) -> DiceRoll {
        let south = self.south();
        let around = self.up.right_hand_around();
        let get_front_index: usize = around
            .iter()
            .enumerate()
            .find_map(|(i, &r)| if front == r { Some(i) } else { None })
            .unwrap();
        around[(get_front_index + index) % 4]
    }
    fn west(&self) -> DiceRoll {
        self.spot_by_index(self.south(), 3)
    }
    fn east(&self) -> DiceRoll {
        self.spot_by_index(self.south(), 1)
    }
    fn spot(&self, dir: Direction) -> DiceRoll {
        match dir {
            Direction::S => self.south(),
            Direction::N => self.north(),
            Direction::E => self.east(),
            Direction::W => self.west(),
        }
    }
    fn roll_south(&mut self) {
        let rev_south = self.south().rev();
        self.south = self.up;
        self.up = rev_south;
    }
    fn roll_north(&mut self) {
        let rev_up = self.up().rev();
        self.up = self.south;
        self.south = rev_up;
    }
    fn roll_west(&mut self) {
        self.up = self.east();
    }
    fn roll_east(&mut self) {
        self.up = self.west();
    }
    fn roll(&mut self, dir: Direction) {
        match dir {
            Direction::S => self.roll_south(),
            Direction::N => self.roll_north(),
            Direction::E => self.roll_east(),
            Direction::W => self.roll_west(),
        }
    }
    fn set_by_roll(&mut self, up: DiceRoll, front: DiceRoll) {
        self.up = up;
        self.south = front;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    S,
    N,
    E,
    W,
}

use std::convert::TryFrom;
impl TryFrom<&str> for Direction {
    type Error = ();
    fn try_from(value: &str) -> Result<Direction, Self::Error> {
        match value {
            "S" => Ok(Direction::S),
            "N" => Ok(Direction::N),
            "E" => Ok(Direction::E),
            "W" => Ok(Direction::W),
            _ => Err(()),
        }
    }
}

struct Dice {
    label_1: usize,
    label_2: usize,
    label_3: usize,
    label_4: usize,
    label_5: usize,
    label_6: usize,
    loc: Location,
}

impl Dice {
    fn up(&self) -> usize {
        match self.loc.up() {
            DiceRoll::R1 => self.label_1,
            DiceRoll::R2 => self.label_2,
            DiceRoll::R3 => self.label_3,
            DiceRoll::R4 => self.label_4,
            DiceRoll::R5 => self.label_5,
            DiceRoll::R6 => self.label_6,
        }
    }
    fn down(&self) -> usize {
        match self.loc.down() {
            DiceRoll::R1 => self.label_1,
            DiceRoll::R2 => self.label_2,
            DiceRoll::R3 => self.label_3,
            DiceRoll::R4 => self.label_4,
            DiceRoll::R5 => self.label_5,
            DiceRoll::R6 => self.label_6,
        }
    }
    fn spot(&self, dir: Direction) -> usize {
        match self.loc.spot(dir) {
            DiceRoll::R1 => self.label_1,
            DiceRoll::R2 => self.label_2,
            DiceRoll::R3 => self.label_3,
            DiceRoll::R4 => self.label_4,
            DiceRoll::R5 => self.label_5,
            DiceRoll::R6 => self.label_6,
        }
    }
    fn get_from_num(&self, i: usize) -> Option<DiceRoll> {
        match i {
            _ if i == self.label_1 => Some(DiceRoll::R1),
            _ if i == self.label_2 => Some(DiceRoll::R2),
            _ if i == self.label_3 => Some(DiceRoll::R3),
            _ if i == self.label_4 => Some(DiceRoll::R4),
            _ if i == self.label_5 => Some(DiceRoll::R5),
            _ if i == self.label_6 => Some(DiceRoll::R6),
            _ => None,
        }
    }
    fn set_by_roll(&mut self, up: DiceRoll, front: DiceRoll) {
        self.loc.set_by_roll(up, front);
    }
    fn roll(&mut self, dir: Direction) {
        self.loc.roll(dir);
    }
}

fn main() {
    let mut dice = input_dice();
    let direction_orders = input_questions();
    for (up_num, south_num) in direction_orders {
        let (r_up, r_south) = (dice.get_from_num(up_num).unwrap(), dice.get_from_num(south_num).unwrap());
        dice.set_by_roll(r_up, r_south);
        println!("{}", dice.spot(Direction::E));
    }
}

fn input_dice() -> Dice {
    let v: Vec<_> = readline()
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect();
    Dice {
        label_1: v[0],
        label_2: v[1],
        label_3: v[2],
        label_4: v[3],
        label_5: v[4],
        label_6: v[5],
        loc: Location::default(),
    }
}

fn input_questions() -> Vec<(usize, usize)> {
    let q = readline().trim().parse::<usize>().unwrap();
    (0..q).map(|_|{
        let v: Vec<usize> = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
        (v[0], v[1])
    }).collect()
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

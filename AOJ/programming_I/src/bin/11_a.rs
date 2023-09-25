fn main() {
    let mut dice = input_dice();
    let direction_orders = input_direction_orders();
    for dir in direction_orders {
        dice.roll(dir);
    }
    println!("{}", dice.up());
}

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
    fn west(&self) -> DiceRoll {
        let south = self.south();
        let around = self.up().right_hand_around();
        let get_south_index: usize = {
            let (i, r) = around
                .iter()
                .enumerate()
                .find(|&(_, r)| self.south == *r)
                .unwrap();
            i
        };
        let west_index = (get_south_index + 3) % 4;
        around[west_index]
    }
    fn east(&self) -> DiceRoll {
        let south = self.south();
        let around = self.up().right_hand_around();
        let get_south_index: usize = {
            let (i, r) = around
                .iter()
                .enumerate()
                .find(|&(_, r)| self.south == *r)
                .unwrap();
            i
        };
        let east_index = (get_south_index + 1) % 4;
        around[east_index]
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
    fn roll(&mut self, dir: Direction) {
        self.loc.roll(dir);
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

fn input_direction_orders() -> Vec<Direction> {
    readline().trim().chars().map(|char: char| Direction::try_from(char.to_string().as_ref()).unwrap()).collect()
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    #[test]
    fn dice_test() {
        let mut dice = Location::default();
        assert_eq!(dice.up(), DiceRoll::R1);
        assert_eq!(dice.south(), DiceRoll::R2);
        assert_eq!(dice.west(), DiceRoll::R4);
        dice.roll_south();
        assert_eq!(dice.up(), DiceRoll::R5);
        assert_eq!(dice.south(), DiceRoll::R1);
        assert_eq!(dice.west(), DiceRoll::R4);
        dice.roll_south();
        assert_eq!(dice.up(), DiceRoll::R6);
        assert_eq!(dice.south(), DiceRoll::R5);
        assert_eq!(dice.west(), DiceRoll::R4);
        dice.roll_south();
        assert_eq!(dice.up(), DiceRoll::R2);
        assert_eq!(dice.south(), DiceRoll::R6);
        assert_eq!(dice.west(), DiceRoll::R4);
        dice.roll_south();

        dice.roll_west();
        assert_eq!(dice.up(), DiceRoll::R3);
        assert_eq!(dice.south(), DiceRoll::R2);
        dice.roll_west();
        assert_eq!(dice.up(), DiceRoll::R6);
        assert_eq!(dice.south(), DiceRoll::R2);
        dice.roll_west();
        assert_eq!(dice.up(), DiceRoll::R4);
        assert_eq!(dice.south(), DiceRoll::R2);
        dice.roll_west();
    }
}

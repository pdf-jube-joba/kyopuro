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

#[derive(Debug, Clone)]
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
    fn roll_as_unmut(&self, dir: Direction) -> Location {
        let mut other: Location = self.clone();
        other.roll(dir);
        other
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

#[derive(Debug, Clone)]
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
    fn roll_as_unmut(&self, dir: Direction) -> Dice {
        let mut other: Dice = self.clone();
        other.loc.roll(dir);
        other
    }
}

fn main() {
    let dices = input_dices();
    let n = dices.len();
    let exists_eq_dice = (0..n).flat_map(move |i| (i+1..n).map(move |j| (i, j))).any(|(i,j)|{
        eq(&dices[i], &dices[j])
    });
    if !exists_eq_dice {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn eq(dice1: &Dice, dice2: &Dice) -> bool {
    let all_combination: Vec<(DiceRoll, DiceRoll)> = vec![
        DiceRoll::R1,
        DiceRoll::R2,
        DiceRoll::R3,
        DiceRoll::R4,
        DiceRoll::R5,
        DiceRoll::R6,
    ].into_iter().flat_map(|roll1| roll1.right_hand_around().into_iter().map(move |roll2| (roll1, roll2))).collect();
    all_combination.into_iter().any(|(up, front)| {
        let mut new_dice1 = dice1.clone();
        new_dice1.set_by_roll(up, front);
        eq_as_located(&new_dice1, dice2)
    })
}

fn eq_as_located(dice1: &Dice, dice2: &Dice) -> bool {
    dice1.up() == dice2.up() &&
    dice1.down() == dice2.down() &&
    dice1.spot(Direction::S) == dice2.spot(Direction::S) &&
    dice1.spot(Direction::N) == dice2.spot(Direction::N) &&
    dice1.spot(Direction::E) == dice2.spot(Direction::E) &&
    dice1.spot(Direction::W) == dice2.spot(Direction::W) 
}

fn input_dices() -> Vec<Dice> {
    let m = readline().trim().parse::<usize>().unwrap();
    (0..m).map(|_| input_dice()).collect()
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

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiceLabel {
    U = 0,
    D = 5,
    F = 1,
    B = 4,
    R = 2,
    L = 3,
}

// relation between labels
impl DiceLabel {
    fn all() -> Vec<Self> {
        vec![
            DiceLabel::U,
            DiceLabel::D,
            DiceLabel::R,
            DiceLabel::L,
            DiceLabel::F,
            DiceLabel::B,
        ]
    }
    fn rev(self) -> DiceLabel {
        match self {
            DiceLabel::U => Self::D,
            DiceLabel::D => Self::U,
            DiceLabel::F => Self::B,
            DiceLabel::B => Self::F,
            DiceLabel::R => Self::L,
            DiceLabel::L => Self::R,
        }
    }
    fn right_hand_around(self) -> Vec<DiceLabel> {
        match self {
            DiceLabel::U => vec![DiceLabel::F, DiceLabel::R, DiceLabel::B, DiceLabel::L],
            DiceLabel::D => vec![DiceLabel::F, DiceLabel::L, DiceLabel::B, DiceLabel::R],
            DiceLabel::F => vec![DiceLabel::U, DiceLabel::L, DiceLabel::D, DiceLabel::R],
            DiceLabel::B => vec![DiceLabel::U, DiceLabel::R, DiceLabel::D, DiceLabel::L],
            DiceLabel::R => vec![DiceLabel::U, DiceLabel::F, DiceLabel::D, DiceLabel::B],
            DiceLabel::L => vec![DiceLabel::U, DiceLabel::B, DiceLabel::D, DiceLabel::F],
        }
    }
}

#[derive(Debug, Clone)]
struct Location {
    up: DiceLabel,
    south: DiceLabel,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            up: DiceLabel::U,
            south: DiceLabel::F,
        }
    }
}

impl Location {
    fn new(up: DiceLabel, south: DiceLabel) -> Location {
        Self { up, south }
    }
    fn get_lotated_label(&self, side: DiceLabel) -> DiceLabel {
        match side {
            DiceLabel::U => self.up,
            DiceLabel::D => self.up.rev(),
            _ => self
                .up
                .right_hand_around()
                .into_iter()
                .cycle()
                .skip_while(|item| *item != self.south)
                .zip(DiceLabel::U.right_hand_around())
                .find_map(|(item1, item2)| if item2 == side { Some(item1) } else { None })
                .unwrap(),
        }
    }
    fn right_hand_spin(self, side: DiceLabel) -> Self {
        match side {
            DiceLabel::U => Self {
                up: self.get_lotated_label(DiceLabel::U),
                south: self.get_lotated_label(DiceLabel::L),
            },
            DiceLabel::D => Self {
                up: self.get_lotated_label(DiceLabel::U),
                south: self.get_lotated_label(DiceLabel::R),
            },
            DiceLabel::R => Self {
                up: self.get_lotated_label(DiceLabel::B),
                south: self.get_lotated_label(DiceLabel::U),
            },
            DiceLabel::L => Self {
                up: self.get_lotated_label(DiceLabel::F),
                south: self.get_lotated_label(DiceLabel::D),
            },
            DiceLabel::F => Self {
                up: self.get_lotated_label(DiceLabel::R),
                south: self.get_lotated_label(DiceLabel::F),
            },
            DiceLabel::B => Self {
                up: self.get_lotated_label(DiceLabel::L),
                south: self.get_lotated_label(DiceLabel::F),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Dice {
    vec: Vec<u8>,
}

impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        DiceLabel::all()
            .into_iter()
            .flat_map(|roll1| {
                roll1
                    .right_hand_around()
                    .into_iter()
                    .map(move |roll2| (roll1, roll2))
            })
            .any(|(up, front)| {
                let putted_dice1 = PuttedDice::new(self.clone(), Location::new(up, front));
                let putted_dice2 = PuttedDice::new(other.clone(), Location::default());
                putted_dice1 == putted_dice2
            })
    }
}

#[derive(Debug, Clone)]
struct PuttedDice {
    dice: Dice,
    loc: Location,
}

impl PuttedDice {
    fn new(dice: Dice, loc: Location) -> PuttedDice {
        Self { dice, loc }
    }
    fn get_by_label(&self, label: DiceLabel) -> u8 {
        self.dice.vec[self.loc.get_lotated_label(label) as usize]
    }
}

impl PartialEq for PuttedDice {
    fn eq(&self, other: &Self) -> bool {
        DiceLabel::all()
            .into_iter()
            .all(|label| self.get_by_label(label) == other.get_by_label(label))
    }
}

fn main() {
    let dices = input_dices();
    let n = dices.len();
    let exists_eq_dice = (0..n)
        .flat_map(move |i| (i + 1..n).map(move |j| (i, j)))
        .any(|(i, j)| dices[i] == dices[j]);
    if !exists_eq_dice {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input_dices() -> Vec<Dice> {
    let m = readline().trim().parse::<usize>().unwrap();
    (0..m).map(|_| input_dice()).collect()
}

fn input_dice() -> Dice {
    let vec: Vec<_> = readline()
        .split_whitespace()
        .map(|str| str.parse::<u8>().unwrap())
        .collect();
    Dice { vec }
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loc_test() {
        let loc = Location::default();
        assert_eq!(loc.get_lotated_label(DiceLabel::U), DiceLabel::U);
        assert_eq!(loc.get_lotated_label(DiceLabel::D), DiceLabel::D);
        assert_eq!(loc.get_lotated_label(DiceLabel::R), DiceLabel::R);
        assert_eq!(loc.get_lotated_label(DiceLabel::L), DiceLabel::L);

        let loc = Location::new(DiceLabel::F, DiceLabel::R);
        assert_eq!(loc.get_lotated_label(DiceLabel::U), DiceLabel::F);
        assert_eq!(loc.get_lotated_label(DiceLabel::D), DiceLabel::B);
        assert_eq!(loc.get_lotated_label(DiceLabel::R), DiceLabel::U);
        assert_eq!(loc.get_lotated_label(DiceLabel::L), DiceLabel::D);

        let loc = Location::new(DiceLabel::D, DiceLabel::F);
        assert_eq!(loc.get_lotated_label(DiceLabel::R), DiceLabel::L);
        assert_eq!(loc.get_lotated_label(DiceLabel::L), DiceLabel::R)
    }
}

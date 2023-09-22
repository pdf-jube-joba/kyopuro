use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Sort {
    S,
    H,
    C,
    D,
}

use std::convert::TryFrom;

impl TryFrom<&str> for Sort {
    type Error = String;
    fn try_from(value: &str) -> Result<Sort, String> {
        match value {
            "S" => Ok(Sort::S),
            "H" => Ok(Sort::H),
            "C" => Ok(Sort::C),
            "D" => Ok(Sort::D),
            _ => Err("".to_string()),
        }
    }
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::S => "S".to_string(),
            Sort::H => "H".to_string(),
            Sort::C => "C".to_string(),
            Sort::D => "D".to_string(),
        }
    }
}

fn main() {
    let mut all_card: Vec<(Sort, usize)> = vec![];
    all_card.extend((1..=13).map(|i| (Sort::S, i)));
    all_card.extend((1..=13).map(|i| (Sort::H, i)));
    all_card.extend((1..=13).map(|i| (Sort::C, i)));
    all_card.extend((1..=13).map(|i| (Sort::D, i)));
    let appered_vec = input();
    for card in all_card {
        if !appered_vec.contains(&card) {
            println!("{} {}", card.0.to_string(), card.1);
        }
    }

}

fn input() -> HashSet<(Sort, usize)> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    let n = string.trim().parse::<usize>().unwrap();

    let mut vec: HashSet<(Sort, usize)> = HashSet::new();

    for _ in 0..n {
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let vec_str: Vec<&str> = string.split_whitespace().collect();
        let sort: Sort = Sort::try_from(vec_str[0]).unwrap();
        let num: usize = vec_str[1].parse().unwrap();
        vec.insert((sort, num));
    }

    vec

}
#[derive(Debug, Clone, PartialEq)]
enum Sort {
    S, H, C, D,
}

use std::str::FromStr;
impl FromStr for Sort {
    type Err = ();
    fn from_str(value: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match value {
            "S" => Ok(Sort::S),
            "H" => Ok(Sort::H),
            "C" => Ok(Sort::C),
            "D" => Ok(Sort::D),
            _ => Err(()),
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
    let cards = input();
    let mut sort_cards = cards.clone();
    quicksort(&mut sort_cards[..]);
    if is_stable(&cards, &sort_cards) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
    for card in sort_cards {
        println!("{} {}", card.0.to_string(), card.1);
    }
}

fn is_stable(before_sort: &[(Sort, usize)], after_sort: &[(Sort, usize)]) -> bool {
    let v: Vec<&[(Sort, usize)]> = {
        let mut ranges = vec![];
        let mut start = 0;

        for i in 0..after_sort.len() - 1 {
            if after_sort[i].1 != after_sort[i+1].1 {
                ranges.push((start..i+1));
                start = i + 1;
            }
        }
        ranges.push((start..after_sort.len()));
        ranges.into_iter().map(|range| &after_sort[range] ).collect()
    };

    for vs in v {
        let sorted: Vec<_> = vs.iter().map(|(sort, _)| sort.clone()).collect();
        let before: Vec<_> = before_sort.iter().filter_map(|card| {
            if vs.contains(card) {
                Some(card.0.clone())
            } else {
                None
            }
        }).collect();
        if sorted != before {
            return false
        }
    }
    true
}

fn quicksort(a: &mut[(Sort, usize)]) {
    if a.len() > 1 {
        let r = a.len() - 1;
        let q = partition(&mut a[..]);
        quicksort(&mut a[0..q]);
        quicksort(&mut a[q+1..=r]);
    }
}

fn partition(a: &mut [(Sort, usize)]) -> usize {
    let r = a.len() - 1;
    let x = a[r].1;
    let mut i: i32 = -1;
    for j in 0..r {
        if a[j].1 <= x {
            i += 1;
            a.swap(i as usize, j);
        }
    }
    a.swap((i + 1) as usize, r);
    (i + 1) as usize
}

fn input() -> Vec<(Sort, usize)> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let n = string.trim().parse::<usize>().unwrap();
    (0..n).map(|_|{
        string.clear();
        std::io::stdin().read_line(&mut string).unwrap();
        let mut s = string.split_whitespace();
        let sort = s.next().unwrap().parse::<Sort>().unwrap();
        let num =  s.next().unwrap().parse::<usize>().unwrap();
        (sort, num)
    }).collect()
}
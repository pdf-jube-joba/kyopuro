use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeCard,
    FullHouse,
    FourCard,
    FiveCard,
}

impl HandType {
    fn judge(a: &Hand) -> Self {
        let mut num = vec![0; 15];
        for i in &a.0 {
            num[*i] += 1;
        }
        let mut nums = [0; 6];
        for i in num {
            nums[i] += 1;
        }
        match nums {
            [_, 0, 0, 0, 0, 1] => HandType::FiveCard,
            [_, 1, 0, 0, 1, 0] => HandType::FourCard,
            [_, 0, 1, 1, 0, 0] => HandType::FullHouse,
            [_, 2, 0, 1, 0, 0] => HandType::ThreeCard,
            [_, 1, 2, 0, 0, 0] => HandType::TwoPair,
            [_, 3, 1, 0, 0, 0] => HandType::OnePair,
            [_, 5, 0, 0, 0, 0] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
    fn judge_part2(a: &Hand) -> Self {
        // num of cards of same number except J
        let mut num = vec![0; 15];
        let mut joker_num = 0;
        for i in &a.0 {
            if *i == 11 {
                joker_num += 1;
            } else {
                num[*i] += 1;
            }
        }
        // each i represents number of kinds (except J) that appered i times
        let mut nums = [0; 6];
        for i in num {
            nums[i] += 1;
        }
        match (joker_num, nums) {
            // 5
            (0, [_, 0, 0, 0, 0, 1])
            | (1, [_, 0, 0, 0, 1, 0])
            | (2, [_, 0, 0, 1, 0, 0])
            | (3, [_, 0, 1, 0, 0, 0])
            | (4, [_, 1, 0, 0, 0, 0])
            | (5, _) => HandType::FiveCard,
            // 4
            (0, [_, 1, 0, 0, 1, 0])
            | (1, [_, 1, 0, 1, 0, 0])
            | (2, [_, 1, 1, 0, 0, 0])
            | (3, [_, 2, 0, 0, 0, 0]) => HandType::FourCard,
            // 3+2
            (0, [_, 0, 1, 1, 0, 0])
                // if 2 or more joker is exists, we can make stronger hands (or we can't make fullhouse)
            | (1, [_, 0, 2, 0, 0, 0]) => HandType::FullHouse,
            // 3
            (0, [_, 2, 0, 1, 0, 0])
            | (1, [_, 2, 1, 0, 0, 0])
                // if 3 or more joker .....
            | (2, [_, 3, 0, 0, 0, 0]) => HandType::ThreeCard,
            // 2 + 2
            (0, [_, 1, 2, 0, 0, 0]) => HandType::TwoPair,
            (0, [_, 3, 1, 0, 0, 0])
            // 2
            | (1, [_, 4, 0, 0, 0, 0]) => HandType::OnePair,
            // no
            (0, [_, 5, 0, 0, 0, 0]) => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(Vec<usize>);

impl Hand {
    fn new(vec: Vec<usize>) -> Self {
        assert_eq!(vec.len(), 5);
        Self(vec)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            HandType::judge(self)
                .cmp(&HandType::judge(other))
                .then(self.0.cmp(&other.0)),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn cmp_part2(hand1: &Hand, hand2: &Hand) -> Ordering {
    let (mut hand1, mut hand2) = (hand1.clone(), hand2.clone());
    // compute hand type
    let type1 = HandType::judge_part2(&hand1);
    let type2 = HandType::judge_part2(&hand2);
    // regard J as 0 (most weak)
    hand1.0.iter_mut().for_each(|i| {
        if *i == 11 {
            *i = 0
        }
    });
    hand2.0.iter_mut().for_each(|i| {
        if *i == 11 {
            *i = 0
        }
    });
    type1.cmp(&type2).then(hand1.0.cmp(&hand2.0))
}

// take ownership
fn compute_part1(mut hands: Vec<(Hand, usize)>) -> usize {
    hands.sort_by_key(|(hand, _bid)| hand.clone());
    let max_rank = hands.len();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_hand, bid))| {
            let rank = i + 1;
            rank * bid
        })
        .sum()
}

fn compute_part2(mut hands: Vec<(Hand, usize)>) -> usize {
    hands.sort_by(|(hand1, _), (hand2, _)| cmp_part2(hand1, hand2));
    let max_rank = hands.len();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_hand, bid))| {
            let rank = i + 1;
            rank * bid
        })
        .sum()
}

fn main() {
    let hands = input();
    println!("{}", compute_part1(hands.clone()));
    println!("{}", compute_part2(hands.clone()));
}

fn input() -> Vec<(Hand, usize)> {
    let string = std::fs::read_to_string("inout/day7.in").unwrap();
    let convert = |s: char| -> usize {
        match s {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ if s.is_ascii_digit() => s.to_digit(10).unwrap() as usize,
            _ => unreachable!("it not digit {}", s),
        }
    };
    string
        .lines()
        .map(|line| {
            let [s0, s1, ..] = &line.split_whitespace().collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            let hands = s0.chars().map(convert).collect::<Vec<_>>();
            let bid = s1.parse::<usize>().unwrap();
            (Hand::new(hands), bid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hand_test() {
        let tests = vec![
            (vec![2, 2, 2, 2, 2], HandType::FiveCard),
            (vec![2, 2, 2, 2, 3], HandType::FourCard),
            (vec![2, 2, 2, 3, 3], HandType::FullHouse),
            (vec![3, 2, 2, 3, 2], HandType::FullHouse),
            (vec![2, 2, 2, 3, 4], HandType::ThreeCard),
            (vec![2, 2, 3, 3, 4], HandType::TwoPair),
            (vec![2, 2, 3, 4, 5], HandType::OnePair),
            (vec![1, 2, 3, 4, 5], HandType::HighCard),
        ];

        for (hands, exp) in tests {
            let hands = Hand::new(hands);
            assert_eq!(HandType::judge(&hands), exp)
        }

        let tests = vec![
            (vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], Ordering::Equal),
            (vec![2, 2, 2, 2, 3], vec![2, 2, 2, 2, 2], Ordering::Less),
            (vec![2, 2, 2, 3, 3], vec![2, 2, 2, 2, 3], Ordering::Less),
            (vec![2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3], Ordering::Less),
            (vec![3, 2, 2, 2, 2], vec![4, 2, 2, 2, 2], Ordering::Less),
            (vec![2, 2, 2, 2, 3], vec![2, 2, 2, 2, 4], Ordering::Less),
        ];
        for (a, b, ord) in tests {
            let a = Hand::new(a);
            let b = Hand::new(b);
            assert_eq!(a.cmp(&b), ord);
        }
    }
    #[test]
    fn example_test() {
        let hands = vec![
            (Hand::new(vec![3, 2, 10, 3, 13]), 765),
            (Hand::new(vec![10, 5, 5, 11, 5]), 684),
            (Hand::new(vec![13, 13, 6, 7, 7]), 28),
            (Hand::new(vec![13, 10, 11, 11, 10]), 220),
            (Hand::new(vec![12, 12, 12, 11, 14]), 483),
        ];
        assert_eq!(compute_part1(hands), 6440);
    }
    #[test]
    fn hand_part2_test() {
        let tests = vec![
            (vec![2, 2, 2, 2, 2], HandType::FiveCard),
            (vec![2, 2, 2, 2, 3], HandType::FourCard),
            (vec![2, 2, 2, 3, 3], HandType::FullHouse),
            (vec![3, 2, 2, 3, 2], HandType::FullHouse),
            (vec![2, 2, 2, 3, 4], HandType::ThreeCard),
            (vec![2, 2, 3, 3, 4], HandType::TwoPair),
            (vec![2, 2, 3, 4, 5], HandType::OnePair),
            (vec![1, 2, 3, 4, 5], HandType::HighCard),
            // joker case
            (vec![11, 11, 11, 11, 11], HandType::FiveCard),
            (vec![11, 11, 11, 11, 2], HandType::FiveCard),
            (vec![11, 11, 11, 2, 2], HandType::FiveCard),
            (vec![11, 11, 11, 2, 3], HandType::FourCard),
            (vec![11, 11, 2, 2, 2], HandType::FiveCard),
            (vec![11, 11, 2, 2, 3], HandType::FourCard),
            (vec![11, 11, 2, 3, 4], HandType::ThreeCard),
            (vec![11, 2, 2, 2, 2], HandType::FiveCard),
            (vec![11, 2, 2, 2, 3], HandType::FourCard),
            (vec![11, 2, 2, 3, 3], HandType::FullHouse),
            (vec![11, 2, 2, 3, 4], HandType::ThreeCard),
            (vec![11, 2, 3, 4, 5], HandType::OnePair),
        ];

        for (hands, exp) in tests {
            let hands = Hand::new(hands);
            assert_eq!(HandType::judge_part2(&hands), exp)
        }

        let tests = vec![
            (vec![2, 2, 2, 2, 3], vec![2, 2, 2, 2, 11], Ordering::Less),
            (vec![2, 2, 2, 2, 3], vec![2, 2, 2, 11, 11], Ordering::Less),
            (vec![2, 3, 4, 5, 6], vec![11, 3, 4, 5, 6], Ordering::Less),
            (vec![11, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], Ordering::Less),
            (vec![11, 2, 2, 2, 3], vec![11, 2, 2, 2, 2], Ordering::Less),
            (vec![11, 2, 2, 3, 3], vec![11, 2, 2, 2, 3], Ordering::Less),
            (vec![11, 2, 2, 2, 2], vec![11, 3, 3, 3, 3], Ordering::Less),
            (vec![11, 2, 2, 2, 2], vec![3, 11, 3, 3, 3], Ordering::Less),
            (vec![3,11, 2, 2, 2], vec![4, 11, 2, 2, 2], Ordering::Less),
            (vec![2, 2, 2, 2, 3], vec![2, 2, 2, 2, 4], Ordering::Less),
        ];
        for (a, b, ord) in tests {
            let a = Hand::new(a);
            let b = Hand::new(b);
            assert_eq!(cmp_part2(&a, &b), ord);
        }
    }
}

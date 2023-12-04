fn card_point(winning: &[usize], hands: &[usize]) -> usize {
    let mut hands = hands.to_owned();
    hands.sort();
    let win_num = winning.iter().filter(|win|{
        hands.binary_search(win).is_ok()
    }).count();
    if win_num ==0 {
        0
    } else {
        2_usize.pow((win_num - 1) as u32)
    }
}

fn compute_part1(game: &Vec<(usize, Vec<usize>, Vec<usize>)>) -> usize {
    game.iter()
        .map(|(_, winning_game, hands)| {
            card_point(winning_game, hands)
        })
        .sum()
}

fn card_match(winning: &[usize], hands: &[usize]) -> usize {
    let mut hands = hands.to_owned();
    hands.sort();
    winning.iter().filter(|win|{
        hands.binary_search(win).is_ok()
    }).count()
}

fn compute_part2(game: &Vec<(usize, Vec<usize>, Vec<usize>)>) -> usize {
    let mut total_card_num = vec![1; game.len()];
    for (i, win, hands) in game {
        let i = *i - 1;
        let win_num = card_match(win, hands);
        let card_num = total_card_num[i];
        for j in i+1..=i+win_num {
            if j < game.len() {
                total_card_num[j] += card_num;
            }
        }
    }
    total_card_num.into_iter().sum()
}

fn main() {
    let game = input();
    let sum = compute_part1(&game);
    println!("{sum}");
    let sum = compute_part2(&game);
    println!("{sum}");
}

fn input() -> Vec<(usize, Vec<usize>, Vec<usize>)> {
    let string = std::fs::read_to_string("inout/day4.in").unwrap();
    string
        .lines()
        .map(|str| {
            let card_split = str.find(':').unwrap();
            let win_split = str.find('|').unwrap();

            let card_num: usize = str[4..card_split].trim().parse::<usize>().unwrap();
            let winning_num: Vec<usize> = str[card_split + 1..win_split]
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            let hands: Vec<usize> = str[win_split + 1..]
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            (card_num, winning_num, hands)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn card_test() {
        let card1 = (vec![1], vec![2]);
        assert_eq!(card_point(&card1.0, &card1.1), 0);

        let card1 = (vec![1], vec![1]);
        assert_eq!(card_point(&card1.0, &card1.1), 1);

        let card1 = (vec![1,2], vec![1,2]);
        assert_eq!(card_point(&card1.0, &card1.1), 2);

        let card1 = (vec![1,2,3], vec![2,3,4]);
        assert_eq!(card_point(&card1.0, &card1.1), 2);
    }
    #[test]
    fn card_part2_test() {
        let game = vec![
            (1, vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            (2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
            (3, vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
            (4, vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]),
            (5, vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
            (6, vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]),
        ];
        assert_eq!(compute_part2(&game), 30);
    }
}
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

fn read_one_game(line: &str) -> (usize, Vec<Vec<(usize, Color)>>) {
    let mut game_infos = line.split(|c| c == ':' || c == ';');
    let game_num: usize = {
        let game_id = game_infos.next().unwrap();
        game_id[5..].parse().unwrap()
    };
    let game = game_infos
        .map(|show| {
            show.split(|c| c == ',')
                .map(|str| {
                    let str = str.trim();
                    let mut split = str.split(' ');
                    let num = split.next().unwrap().parse::<usize>().unwrap();
                    let col = split.next().unwrap().parse::<Color>().unwrap();
                    (num, col)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (game_num, game)
}

fn valid_num_col((num, col): &(usize, Color)) -> bool {
    match col {
        Color::Red => *num <= 12,
        Color::Green => *num <= 13,
        Color::Blue => *num <= 14,
    }
}

fn valid_game(info: &Vec<Vec<(usize, Color)>>) -> bool {
    for show in info {
        for num_col in show {
            if !valid_num_col(num_col) {
                return false;
            }
        }
    }
    true
}

fn compute_part1(content: &str) -> usize {
    content
        .lines()
        .filter_map(|line| {
            let (game_num, game_info) = read_one_game(line);
            if valid_game(&game_info) {
                Some(game_num)
            } else {
                None
            }
        })
        .sum()
}

fn compute_power(game_info: &Vec<Vec<(usize, Color)>>) -> usize {
    let mut need_red = 0; 
    let mut need_green = 0;
    let mut need_blue = 0;
    for show in game_info {
        for (num, col) in show {
            match col {
                Color::Red => {
                    need_red = std::cmp::max(need_red, *num);
                }
                Color::Green => {
                    need_green = std::cmp::max(need_green, *num);
                }
                Color::Blue => {
                    need_blue = std::cmp::max(need_blue, *num);
                }
            }
        }
    }
    need_red * need_green * need_blue
}

fn compute_part2(content: &str) -> usize {
    content.lines().map(|line|{
        let (_, game_info) = read_one_game(line);
        compute_power(&game_info)
    }).sum()
}

fn main() {
    let content = input();
    let sum = compute_part1(&content);
    println!("{sum}");
    let sum = compute_part2(&content);
    println!("{sum}");
}

fn input() -> String {
    std::fs::read_to_string("inout/day2.in").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_one_game_test() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let v = (
            1,
            vec![
                vec![(3, Color::Blue), (4, Color::Red)],
                vec![(1, Color::Red), (2, Color::Green), (6, Color::Blue)],
                vec![(2, Color::Green)],
            ],
        );
        assert_eq!(read_one_game(s), v);
    }
    #[test]
    fn validation_game_test() {
        let game = vec![];
        assert!(valid_game(&game));

        let game = vec![
            vec![(3, Color::Blue), (4, Color::Red)],
            vec![(1, Color::Red), (2, Color::Green), (6, Color::Blue)],
            vec![(2, Color::Green)],
        ];
        assert!(valid_game(&game));
        let game = vec![vec![(20, Color::Red)]];
        assert!(!valid_game(&game));
        let game = vec![
            vec![(1, Color::Green), (3, Color::Red), (6, Color::Blue)],
            vec![(3, Color::Green), (6, Color::Red)],
            vec![(3, Color::Green), (15, Color::Blue), (14, Color::Red)],
        ];
        assert!(!valid_game(&game));
    }
    #[test]
    fn power_test() {
        let game = vec![];
        assert_eq!(compute_power(&game), 0);
        let game = vec![
            vec![(3, Color::Blue), (4, Color::Red)],
            vec![(1, Color::Red), (2, Color::Green), (6, Color::Blue)],
            vec![(2, Color::Green)],
        ];
        assert_eq!(compute_power(&game), 6 * 4 * 2);
    }
}

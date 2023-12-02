fn compute_part1(str: &str) -> usize {
    str.lines().map(|line|{
        let v = line.chars().filter_map(|c| if c.is_ascii_digit() {
            Some(c.to_digit(10).unwrap() as usize)
        } else {
            None
        }).collect::<Vec<_>>();
        v[0] * 10 + v[v.len() - 1]
    }).sum()
}

fn parse_first(str: &[char]) -> Option<usize> {
    match str {
        [c, ..] if c.is_ascii_digit() => Some(c.to_digit(10).unwrap() as usize),
        ['o', 'n', 'e', ..] => Some(1),
        ['t', 'w', 'o', ..] => Some(2),
        ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
        ['f', 'o', 'u', 'r', ..] => Some(4),
        ['f', 'i', 'v', 'e', ..] => Some(5),
        ['s', 'i', 'x', ..] => Some(6),
        ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
        ['e', 'i', 'g', 'h', 't', ..] => Some(8),
        ['n', 'i', 'n', 'e', ..] => Some(9),
        _ => None,
    }
}

fn compute_part2(str: &str) -> usize {
    str.lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();
            let first_digit = {
                let mut i = 0;
                loop {
                    assert!(i < chars.len());
                    if let Some(digit) = parse_first(&chars[i..]) {
                        break digit;
                    }
                    i += 1;
                }
            };
            let last_digit = {
                let mut i = chars.len() - 1;
                loop {
                    if let Some(digit) = parse_first(&chars[i..]) {
                        break digit;
                    }
                    i -= 1;
                }
            };
            first_digit * 10 + last_digit
        })
        .sum()
}

fn main() {
    let content = input();
    let sum = compute_part1(&content);
    println!("{sum}");
}

fn input() -> String {
    std::fs::read_to_string("inout/day1.in").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_part1() {
        let str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let sum = compute_part1(str);
        assert_eq!(sum, 142);
    }
    #[test]
    fn parse_test() {
        assert_eq!(parse_first(&"".chars().collect::<Vec<_>>()), None);
        assert_eq!(parse_first(&"a".chars().collect::<Vec<_>>()), None);
        assert_eq!(parse_first(&"o".chars().collect::<Vec<_>>()), None);
        assert_eq!(parse_first(&"1".chars().collect::<Vec<_>>()), Some(1));
        assert_eq!(parse_first(&"2".chars().collect::<Vec<_>>()), Some(2));
        assert_eq!(parse_first(&"3".chars().collect::<Vec<_>>()), Some(3));
        assert_eq!(parse_first(&"4".chars().collect::<Vec<_>>()), Some(4));
        assert_eq!(parse_first(&"5".chars().collect::<Vec<_>>()), Some(5));
        assert_eq!(parse_first(&"6".chars().collect::<Vec<_>>()), Some(6));
        assert_eq!(parse_first(&"7".chars().collect::<Vec<_>>()), Some(7));
        assert_eq!(parse_first(&"8".chars().collect::<Vec<_>>()), Some(8));
        assert_eq!(parse_first(&"9".chars().collect::<Vec<_>>()), Some(9));
        assert_eq!(parse_first(&"one".chars().collect::<Vec<_>>()), Some(1));
        assert_eq!(parse_first(&"two".chars().collect::<Vec<_>>()), Some(2));
        assert_eq!(parse_first(&"three".chars().collect::<Vec<_>>()), Some(3));
        assert_eq!(parse_first(&"four".chars().collect::<Vec<_>>()), Some(4));
        assert_eq!(parse_first(&"five".chars().collect::<Vec<_>>()), Some(5));
        assert_eq!(parse_first(&"six".chars().collect::<Vec<_>>()), Some(6));
        assert_eq!(parse_first(&"seven".chars().collect::<Vec<_>>()), Some(7));
        assert_eq!(parse_first(&"eight".chars().collect::<Vec<_>>()), Some(8));
        assert_eq!(parse_first(&"nine".chars().collect::<Vec<_>>()), Some(9));
    }
    #[test]
    fn compute_test() {
        let str = "12";
        assert_eq!(compute_part2(str), 12);
        let str = "1234";
        assert_eq!(compute_part2(str), 14);
        let str = "1";
        assert_eq!(compute_part2(str), 11);
        let str = "onetwo";
        assert_eq!(compute_part2(str), 12);
        let str = "one2three4";
        assert_eq!(compute_part2(str), 14);
        let str = "twone";
        assert_eq!(compute_part2(str), 21);
        let str = "one";
        assert_eq!(compute_part2(str), 11);
    }
    #[test]
    fn example_test_part2() {
        let str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(compute_part2(str), 281);
    }
}
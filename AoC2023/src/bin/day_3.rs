use std::ops::RangeInclusive;

fn collect_num_of_line(content: &Vec<Vec<Input>>, i: usize) -> Vec<(RangeInclusive<usize>, usize)> {
    let mut v = vec![];
    let mut start_index_and_count: Option<(usize, usize)> = None;
    for (j, input) in content[i].iter().enumerate() {
        match input {
            Input::Period | Input::Symbol | Input::Gear => {
                if let Some((index, count)) = start_index_and_count {
                    v.push((index..=j - 1, count));
                    start_index_and_count = None;
                }
            }
            Input::Number(num) => {
                if let Some((_, ref mut count)) = start_index_and_count {
                    *count = *count * 10 + num;
                } else {
                    start_index_and_count = Some((j, *num));
                }
            }
        }
    }
    if let Some((index, count)) = start_index_and_count {
        v.push((index..=content[i].len() - 1, count));
    }
    v
}

fn around_of_index(content: &Vec<Vec<Input>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];
    let di = [-1, 0, 1];
    let dj = [-1, 0, 1];
    for di in di {
        let ni = di + (i as isize);
        if 0 <= ni && ni < content.len() as isize {
            for dj in dj {
                let nj = dj + (j as isize);
                if 0 <= nj && nj < content[ni as usize].len() as isize {
                    v.push((ni as usize, nj as usize));
                }
            }
        }
    }
    v
}

fn has_symbol_in_around_num(
    content: &Vec<Vec<Input>>,
    i: usize,
    range: RangeInclusive<usize>,
) -> bool {
    for j in range {
        for (ni, nj) in around_of_index(content, i, j) {
            if content[ni][nj].is_symbol() {
                return true;
            }
        }
    }
    false
}

fn compute_part1(content: &Vec<Vec<Input>>) -> usize {
    (0..content.len())
        .map(|i| {
            collect_num_of_line(content, i)
                .into_iter()
                .filter_map(|(range, num)| {
                    if has_symbol_in_around_num(content, i, range) {
                        Some(num)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn num_around_index(content: &Vec<Vec<Input>>, i: usize, j: usize) -> Vec<usize> {
    let mut v = vec![];
    let is_neighbor =
        |range: RangeInclusive<usize>| *range.end() + 1 >= j && j + 1 >= *range.start();
    // check all range in i-1 line
    if 0 < i {
        v.extend(
            collect_num_of_line(content, i - 1)
                .into_iter()
                .filter_map(|(range, num)| if is_neighbor(range) { Some(num) } else { None }),
        );
    }
    // check all range in i line
    v.extend(
        collect_num_of_line(content, i)
            .into_iter()
            .filter_map(|(range, num)| if is_neighbor(range) { Some(num) } else { None }),
    );
    // check all range in i+1 range
    if i + 1 < content.len() {
        v.extend(
            collect_num_of_line(content, i + 1)
                .into_iter()
                .filter_map(|(range, num)| if is_neighbor(range) { Some(num) } else { None }),
        );
    }
    v
}

fn compute_part2(content: &Vec<Vec<Input>>) -> usize {
    (0..content.len())
        .flat_map(|i| (0..content[i].len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            if content[i][j] == Input::Gear {
                let v = num_around_index(content, i, j);
                if v.len() == 2 {
                    Some(v[0] * v[1])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let content = input();
    let content = parse(&content);
    println!("{}", compute_part1(&content));
    println!("{}", compute_part2(&content));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Input {
    Period,
    Symbol,
    Number(usize),
    Gear,
}

impl Input {
    fn is_symbol(&self) -> bool {
        matches!(self, Input::Gear | Input::Symbol)
    }
}

fn parse(content: &str) -> Vec<Vec<Input>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Input::Period,
                    '*' => Input::Gear,
                    _ if c.is_ascii_digit() => Input::Number(c.to_digit(10).unwrap() as usize),
                    _ => Input::Symbol,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn input() -> String {
    std::fs::read_to_string("inout/day3.in").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collect_test() {
        let s = "1";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![(0..=0, 1)]);

        let s = "*";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![]);

        let s = ".";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![]);

        let s = "123";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![(0..=2, 123)]);

        let s = ".123";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![(1..=3, 123)]);

        let s = "123.";
        let input = parse(s);
        assert_eq!(collect_num_of_line(&input, 0), vec![(0..=2, 123)]);

        let s = "123..4*5...6";
        let input = parse(s);
        assert_eq!(
            collect_num_of_line(&input, 0),
            vec![(0..=2, 123), (5..=5, 4), (7..=7, 5), (11..=11, 6)]
        );
    }
    #[test]
    fn around_symbol_test() {
        let s = "1";
        let input = parse(s);
        assert!(!has_symbol_in_around_num(&input, 0, 0..=0));

        let s = "1.";
        let input = parse(s);
        assert!(!has_symbol_in_around_num(&input, 0, 0..=0));

        let s = "1*";
        let input = parse(s);
        assert!(has_symbol_in_around_num(&input, 0, 0..=0));

        let s = "*1";
        let input = parse(s);
        assert!(has_symbol_in_around_num(&input, 0, 1..=1));

        let s = "12*";
        let input = parse(s);
        assert!(has_symbol_in_around_num(&input, 0, 0..=1));

        let s = "..*\n12.\n...";
        let input = parse(s);
        assert!(has_symbol_in_around_num(&input, 1, 0..=1));

        let s = "...\n12.\n...";
        let input = parse(s);
        assert!(!has_symbol_in_around_num(&input, 1, 0..=1));
    }
    #[test]
    fn sum_test() {
        let s = "...\n12.\n...";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 0);

        let s = "...\n12*\n...";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 12);

        let s = "..*\n12.\n...";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 12);

        let s = "...\n12.\n*..";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 12);

        let s = "1*1\n*1*\n1*1";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 5);

        let s = "1*1\n.1.\n1.1";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 3);

        let s = "1*1\n.1.\n1.1";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 3);

        let s = "123\n.*.\n123";
        let input = parse(s);
        assert_eq!(compute_part1(&input), 123 * 2);
    }
}

use std::collections::HashMap;

fn main() {
    let input = input();
    // println!("{}", compute_part1(input.clone()));
    println!("{}", compute_part2(input))
}

fn compute_part1(input: Vec<(Vec<InputSign>, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(r, g)| {
            let r: Vec<Option<State>> = r
                .into_iter()
                .map(|input_sign| match input_sign {
                    InputSign::Damaged => Some(State::Broken),
                    InputSign::Rational => Some(State::Rational),
                    InputSign::Unknown => None,
                })
                .collect();
            count_arrangement(&r, &g)
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    Rational,
    Broken,
}

fn count_arrangement(record: &Vec<Option<State>>, group: &Vec<usize>) -> usize {
    let total_len = record.len();
    let none_num = record.iter().filter(|state| state.is_none()).count();
    (0..(1 << none_num))
        .map(|bit_pattern| {
            let mut count = 0;
            (0..total_len)
                .map(|k| {
                    if let Some(state) = &record[k] {
                        state.clone()
                    } else if bit_pattern & (1 << count) != 0 {
                        count += 1;
                        State::Rational
                    } else {
                        count += 1;
                        State::Broken
                    }
                })
                .collect::<Vec<State>>()
        })
        .filter(|pattern| {
            // println!(
            //     "{}",
            //     pattern
            //         .iter()
            //         .map(|state| if *state == State::Broken { '#' } else { '.' })
            //         .collect::<String>()
            // );
            pattern
                .split(|state| *state == State::Rational)
                .map(|cont_ratio| cont_ratio.len())
                .filter(|&len| len != 0)
                .collect::<Vec<_>>()
                == *group
        })
        .count()
}

fn can_be_rational(a: &Option<State>) -> bool {
    matches!(a, None | Some(State::Rational))
}

fn can_be_broken(a: &Option<State>) -> bool {
    matches!(a, None | Some(State::Broken))
}

fn count_arrangement_rec_dp<'a>(
    record: &'a [Option<State>],
    group: &'a [usize],
    dp: &mut HashMap<(&'a [Option<State>], &'a [usize]), usize>,
) -> usize {
    if let Some(val) = dp.get(&(record, group)) {
        return *val;
    }
    let val: usize = if group.is_empty() {
        if record.iter().all(can_be_rational) {
            1
        } else {
            0
        }
    } else if record.len() <= group[0] {
        0
    } else {
        // we can access record[group[0]]
        let rational_case = if can_be_rational(&record[0]) {
            count_arrangement_rec_dp(&record[1..], group, dp)
        } else {
            0
        };
        let broken_case = if record[0..group[0]].iter().all(can_be_broken)
            && can_be_rational(&record[group[0]])
        {
            count_arrangement_rec_dp(&record[group[0] + 1..], &group[1..], dp)
        } else {
            0
        };
        rational_case + broken_case
    };
    dp.insert((record, group), val);
    val
}

// please padding 1 '.' at last
fn count_arrangement_rec(record: &[Option<State>], group: &[usize]) -> usize {
    let mut dp = HashMap::new();
    count_arrangement_rec_dp(record, group, &mut dp)
}

fn repeat_and_padding((record, group): (Vec<Option<State>>, Vec<usize>)) -> (Vec<Option<State>>, Vec<usize>) {
    let record: Vec<_> = vec![
        record.clone(),
        vec![None],
        record.clone(),
        vec![None],
        record.clone(),
        vec![None],
        record.clone(),
        vec![None],
        record.clone(),
        vec![Some(State::Rational)],
    ]
    .into_iter()
    .flatten()
    .collect();
    let group: Vec<_> = vec![
        group.clone(),
        group.clone(),
        group.clone(),
        group.clone(),
        group.clone(),
    ]
    .into_iter()
    .flatten()
    .collect();
    (record, group)
}

fn compute_part2(input: Vec<(Vec<InputSign>, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(record, group)| {
            let record: Vec<_> = record
                .into_iter()
                .map(|state| match state {
                    InputSign::Damaged => Some(State::Broken),
                    InputSign::Rational => Some(State::Rational),
                    InputSign::Unknown => None,
                })
                .collect();
            let (record, group) = repeat_and_padding((record, group));
            let count = count_arrangement_rec(&record, &group);
            count
        })
        .sum()
}

fn print(a: &[Option<State>]) {
    print!(
        "{}",
        a.iter()
            .map(|state| match state {
                None => '?',
                Some(State::Broken) => '#',
                Some(State::Rational) => '.',
            })
            .collect::<String>()
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InputSign {
    Rational,
    Damaged,
    Unknown,
}

fn input() -> Vec<(Vec<InputSign>, Vec<usize>)> {
    let string = std::fs::read_to_string("inout/day12.in").unwrap();
    string
        .lines()
        .map(|line| {
            let space = line.find(' ').unwrap();
            let v1: Vec<_> = line[..space]
                .chars()
                .map(|char| match char {
                    '.' => InputSign::Rational,
                    '#' => InputSign::Damaged,
                    '?' => InputSign::Unknown,
                    _ => unreachable!(),
                })
                .collect();
            let v2: Vec<_> = line[space..]
                .trim()
                .split(',')
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            (v1, v2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_arrangement_test() {
        let r = vec![
            None,
            None,
            None,
            Some(State::Rational),
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
        ];
        let g = vec![1, 1, 3];
        assert_eq!(count_arrangement(&r, &g), 1);

        let r = vec![
            Some(State::Rational),
            None,
            None,
            Some(State::Rational),
            Some(State::Rational),
            None,
            None,
            Some(State::Rational),
            Some(State::Rational),
            Some(State::Rational),
            None,
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Rational),
        ];
        let g = vec![1, 1, 3];
        assert_eq!(count_arrangement(&r, &g), 4);

        let r = vec![
            None,
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let g = vec![3, 2, 1];
        assert_eq!(count_arrangement(&r, &g), 10);
    }
    #[test]
    fn count_arrangement_rec_test() {
        let r = vec![
            Some(State::Rational),
            Some(State::Rational), // padding
        ];
        let g = vec![];
        assert_eq!(count_arrangement_rec(&r, &g), 1);

        let r = vec![
            Some(State::Rational),
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Rational), // padding
        ];
        let g = vec![3];
        assert_eq!(count_arrangement_rec(&r, &g), 1);

        let r = vec![
            None,
            None,
            None,
            Some(State::Rational),
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Rational), // padding
        ];
        let g = vec![1, 1, 3];
        assert_eq!(count_arrangement_rec(&r, &g), 1);

        let r = vec![
            Some(State::Rational),
            None,
            None,
            Some(State::Rational),
            Some(State::Rational),
            None,
            None,
            Some(State::Rational),
            Some(State::Rational),
            Some(State::Rational),
            None,
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Rational),
            Some(State::Rational), // padding
        ];
        let g = vec![1, 1, 3];
        assert_eq!(count_arrangement_rec(&r, &g), 4);

        let r = vec![
            None,
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(State::Rational), // padding
        ];
        let g = vec![3, 2, 1];
        assert_eq!(count_arrangement_rec(&r, &g), 10);
    }
    #[test]
    fn larget_test() {
        let r = vec![
            None,
            Some(State::Broken),
            Some(State::Broken),
            Some(State::Broken),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let g = vec![3, 2, 1];
        let (r, g) = repeat_and_padding((r, g));
        assert_eq!(count_arrangement_rec(&r, &g), 506250);
    }
}

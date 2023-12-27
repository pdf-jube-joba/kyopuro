use std::collections::HashMap;

fn main() {
    let input = input();
    println!("{}", compute_part2(input));
}

fn compute_part1(mut input: Vec<Vec<InputSign>>) -> usize {
    tilt_north(&mut input);
    compute_load(&input)
}

fn tilt(a: &mut Vec<InputSign>) {
    let mut c = if a[0] == InputSign::Space {
        Some(0)
    } else {
        None
    };
    let mut load = 0;
    for i in 0..a.len() {
        match &a[i] {
            InputSign::Space if c.is_none() => {
                c = Some(i);
            }
            InputSign::Cube => {
                c = None;
            }
            InputSign::Round => {
                if let Some(cin) = c {
                    a[i] = InputSign::Space;
                    a[cin] = InputSign::Round;
                    load += i - cin;
                    if a[cin + 1] == InputSign::Space {
                        c = Some(cin + 1);
                    } else {
                        c = None;
                    }
                }
            }
            _ => {}
        }
    }
}

fn print_fmt(input: &Vec<Vec<InputSign>>) {
    for line in input {
        println!(
            "{}",
            line.iter()
                .map(|sign| match sign {
                    InputSign::Cube => '#',
                    InputSign::Round => 'O',
                    InputSign::Space => '.',
                })
                .collect::<String>()
        )
    }
    println!()
}

fn compute_load(dish: &Vec<Vec<InputSign>>) -> usize {
    let row_num = dish.len();
    (0..row_num)
        .map(|i| {
            (row_num - i)
                * dish[i]
                    .iter()
                    .filter(|sign| **sign == InputSign::Round)
                    .count()
        })
        .sum()
}

fn tilt_north(input: &mut Vec<Vec<InputSign>>) {
    let col_num = input[0].len();
    let row_num = input.len();
    for j in 0..col_num {
        let mut cols: Vec<_> = (0..row_num).map(|i| input[i][j].clone()).collect();
        tilt(&mut cols);
        for (i, sign) in cols.into_iter().enumerate() {
            input[i][j] = sign;
        }
    }
}

fn tilt_south(input: &mut Vec<Vec<InputSign>>) {
    let col_num = input[0].len();
    let row_num = input.len();
    for j in 0..col_num {
        let mut cols: Vec<_> = (0..row_num).map(|i| input[i][j].clone()).collect();
        cols.reverse();
        tilt(&mut cols);
        cols.reverse();
        for (i, sign) in cols.into_iter().enumerate() {
            input[i][j] = sign;
        }
    }
}

fn tile_west(input: &mut Vec<Vec<InputSign>>) {
    let col_num = input[0].len();
    let row_num = input.len();
    for j in 0..row_num {
        tilt(&mut input[j]);
    }
}

fn tile_east(input: &mut Vec<Vec<InputSign>>) {
    let col_num = input[0].len();
    let row_num = input.len();
    for j in 0..row_num {
        input[j].reverse();
        tilt(&mut input[j]);
        input[j].reverse();
    }
}

fn spin_cycle(input: &mut Vec<Vec<InputSign>>) {
    tilt_north(input);
    tile_west(input);
    tilt_south(input);
    tile_east(input);
}

fn compute_part2(mut input: Vec<Vec<InputSign>>) -> usize {
    let mut pred = HashMap::new();
    pred.insert(input.clone(), 0);
    for i in 1..=1_000_000_000 {
        println!("{i}");
        spin_cycle(&mut input);
        // input is i cycled here
        if let Some(p) = pred.get(&input) {
            // p + (i - p) * QUO + REM = 1_000_000_000
            let rem = (1_000_000_000 - p) % (i - p);

            for _ in 0..rem {
                print_fmt(&input);
                spin_cycle(&mut input);
            }
            print_fmt(&input);
            return compute_load(&input);
        }
        pred.insert(input.clone(), i);
    }
    compute_load(&input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InputSign {
    Space,
    Cube,
    Round,
}

fn input() -> Vec<Vec<InputSign>> {
    let string = std::fs::read_to_string("inout/day14.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| match char {
                    '.' => InputSign::Space,
                    '#' => InputSign::Cube,
                    'O' => InputSign::Round,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tilt_test() {
        let mut v = vec![InputSign::Space];
        tilt(&mut v);
        let e = vec![InputSign::Space];
        assert_eq!(v, e);

        let mut v = vec![InputSign::Space, InputSign::Round];
        tilt(&mut v);
        let e = vec![InputSign::Round, InputSign::Space];
        assert_eq!(v, e);

        let mut v = vec![InputSign::Cube, InputSign::Space, InputSign::Round];
        tilt(&mut v);
        let e = vec![InputSign::Cube, InputSign::Round, InputSign::Space];
        assert_eq!(v, e);

        let mut v = vec![
            InputSign::Space,
            InputSign::Cube,
            InputSign::Space,
            InputSign::Round,
            InputSign::Round,
        ];
        tilt(&mut v);
        let e = vec![
            InputSign::Space,
            InputSign::Cube,
            InputSign::Round,
            InputSign::Round,
            InputSign::Space,
        ];
        assert_eq!(v, e);

        let mut v = vec![
            InputSign::Round,
            InputSign::Space,
            InputSign::Space,
            InputSign::Round,
            InputSign::Space,
            InputSign::Round,
        ];
        tilt(&mut v);
        let e = vec![
            InputSign::Round,
            InputSign::Round,
            InputSign::Round,
            InputSign::Space,
            InputSign::Space,
            InputSign::Space,
        ];
        assert_eq!(v, e);
    }
    #[test]
    fn load_test() {
        let v = vec![
            vec![InputSign::Round],
            vec![InputSign::Round],
            vec![InputSign::Round],
            vec![InputSign::Cube],
        ];
        assert_eq!(compute_load(&v), 4 + 3 + 2)
    }
    #[test]
    fn cycle_test() {
        fn read_from_string(string: String) -> Vec<Vec<InputSign>> {
            string
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|char| match char {
                            'O' => InputSign::Round,
                            '.' => InputSign::Space,
                            '#' => InputSign::Cube,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        }
        let v0 = "O....#....
                        O.OO#....#
                        .....##...
                        OO.#O....O
                        .O.....O#.
                        O.#..O.#.#
                        ..O..#O..O
                        .......O..
                        #....###..
                        #OO..#....";
        let mut v0 = read_from_string(v0.to_string());
        let v1 = ".....#....
                        ....#...O#
                        ...OO##...
                        .OO#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#....
                        ......OOOO
                        #...O###..
                        #..OO#....";
        let mut v1 = read_from_string(v1.to_string());
        let v2 = ".....#....
                        ....#...O#
                        .....##...
                        ..O#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#...O
                        .......OOO
                        #..OO###..
                        #.OOO#...O";
        let mut v2 = read_from_string(v2.to_string());
        let v3 = ".....#....
                        ....#...O#
                        .....##...
                        ..O#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#...O
                        .......OOO
                        #...O###.O
                        #.OOO#...O";
        let mut v3 = read_from_string(v3.to_string());
        spin_cycle(&mut v0);
        assert_eq!(v1, v0);
        spin_cycle(&mut v0);
        assert_eq!(v2, v0);
        spin_cycle(&mut v0);
        assert_eq!(v3, v0);

        let v0 = "O....#....
                        O.OO#....#
                        .....##...
                        OO.#O....O
                        .O.....O#.
                        O.#..O.#.#
                        ..O..#O..O
                        .......O..
                        #....###..
                        #OO..#....";
        let v0 = read_from_string(v0.to_string());
        assert_eq!(compute_part2(v0), 64);
    }
}

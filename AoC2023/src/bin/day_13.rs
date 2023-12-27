fn main() {
    let input = input();
    println!("{}", compute_part2(input.clone()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sign {
    Dot,
    Sharp,
}

impl From<Sign> for char {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Dot => '.',
            Sign::Sharp => '#',
        }
    }
}

// mirrors is between c col and (c+1) col
fn col_reflection_at(v: &Vec<Vec<Sign>>, c: usize) -> bool {
    for i in 0..std::cmp::min(v[0].len() - c - 1, c + 1) {
        for j in 0..v.len() {
            if v[j][c + i + 1] != v[j][c - i] {
                return false;
            }
        }
    }
    true
}

// mirrors is between c col and (c+1) col
fn col_reflection_with_smudge(v: &Vec<Vec<Sign>>, c: usize) -> bool {
    let mut smudgeuse = false;
    for i in 0..std::cmp::min(v[0].len() - c - 1, c + 1) {
        for j in 0..v.len() {
            if v[j][c + i + 1] != v[j][c - i] {
                if !smudgeuse {
                    smudgeuse = true;
                } else {
                    return false;
                }
            }
        }
    }
    smudgeuse
}

fn row_reflection_at(v: &Vec<Vec<Sign>>, r: usize) -> bool {
    for i in 0..std::cmp::min(v.len() - r - 1, r + 1) {
        for j in 0..v[0].len() {
            if v[r + i + 1][j] != v[r - i][j] {
                return false;
            }
        }
    }
    true
}

fn row_reflection_with_smudge(v: &Vec<Vec<Sign>>, r: usize) -> bool {
    let mut smudgeuse = false;
    for i in 0..std::cmp::min(v.len() - r - 1, r + 1) {
        for j in 0..v[0].len() {
            if v[r + i + 1][j] != v[r - i][j] {
                if !smudgeuse {
                    smudgeuse = true;
                } else {
                    return false;
                }
            }
        }
    }
    smudgeuse
}

fn compute_part1(input: Vec<Vec<Vec<Sign>>>) -> usize {
    input
        .into_iter()
        .map(|pattern| {
            for r in 0..pattern.len() - 1 {
                if row_reflection_at(&pattern, r) {
                    return (r, true);
                }
            }
            for c in 0..pattern[0].len() {
                if col_reflection_at(&pattern, c) {
                    return (c, false);
                }
            }
            unreachable!()
        })
        .map(|(num, flag)| {
            if flag {
                println!("row {num}");
                100 * (num + 1)
            } else {
                println!("col {num}");
                num + 1
            }
        })
        .sum()
}

fn compute_part2(input: Vec<Vec<Vec<Sign>>>) -> usize {
    input
    .into_iter()
    .map(|pattern| {
        for r in 0..pattern.len() - 1 {
            if row_reflection_with_smudge(&pattern, r) {
                return (r, true);
            }
        }
        for c in 0..pattern[0].len() {
            if col_reflection_with_smudge(&pattern, c) {
                return (c, false);
            }
        }
        unreachable!()
    })
    .map(|(num, flag)| {
        if flag {
            println!("row {num}");
            100 * (num + 1)
        } else {
            println!("col {num}");
            num + 1
        }
    })
    .sum()
}

fn input() -> Vec<Vec<Vec<Sign>>> {
    let string = std::fs::read_to_string("inout/day13.in").unwrap();
    string
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|char| match char {
                            '#' => Sign::Sharp,
                            '.' => Sign::Dot,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reflection_test() {
        let pattern = vec![vec![Sign::Sharp, Sign::Dot, Sign::Dot, Sign::Sharp]];
        assert!(!col_reflection_at(&pattern, 0));
        assert!(col_reflection_at(&pattern, 1));
        assert!(!col_reflection_at(&pattern, 2));
        let pattern = vec![
            vec![Sign::Sharp, Sign::Dot, Sign::Dot, Sign::Sharp],
            vec![Sign::Sharp, Sign::Dot, Sign::Dot, Sign::Sharp],
        ];
        assert!(!col_reflection_at(&pattern, 0));
        assert!(col_reflection_at(&pattern, 1));
        assert!(!col_reflection_at(&pattern, 2));
        assert!(row_reflection_at(&pattern, 0));
    }
}

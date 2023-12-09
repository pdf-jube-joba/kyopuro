fn next_extrapolate(a: &[isize]) -> isize {
    if a.is_empty() {
        unreachable!("input should not reach empty sequence");
    }
    if a.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<isize> = (0..a.len() - 1).map(|i| a[i+1] - a[i]).collect();
    let next_of_diff = next_extrapolate(&diffs);
    a.last().unwrap() + next_of_diff
}

fn compute_part1(a: &Vec<Vec<isize>>) -> isize {
    a.iter().map(|vec| next_extrapolate(vec)).sum()
}

fn prev_extrapolate(a: &[isize]) -> isize {
    if a.is_empty() {
        unreachable!("input should not reach empty sequence");
    }
    if a.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<isize> = (0..a.len() - 1).map(|i| a[i+1] - a[i]).collect();
    let prev_of_diff = prev_extrapolate(&diffs);
    a.first().unwrap() - prev_of_diff
}

fn compute_part2(a: &Vec<Vec<isize>>) -> isize {
    a.iter().map(|vec| prev_extrapolate(vec)).sum()
}

fn main() {
    let input = input();
    println!("{}", compute_part1(&input));
    println!("{}", compute_part2(&input));
}

fn input() -> Vec<Vec<isize>> {
    let string = std::fs::read_to_string("inout/day9.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_extrapolate_test() {
        let a = vec![3,3];
        assert_eq!(next_extrapolate(&a), 3);

        let a = vec![3,4,5];
        assert_eq!(next_extrapolate(&a), 6);

        let a = vec![3,4,6,9];
        assert_eq!(next_extrapolate(&a), 13);
    }

    #[test]
    fn prev_extrapolate_test() {
        let a = vec![3,3];
        assert_eq!(prev_extrapolate(&a), 3);

        let a = vec![3,4,5];
        assert_eq!(prev_extrapolate(&a), 2);

        let a = vec![3,4,6,9];
        assert_eq!(prev_extrapolate(&a), 3);
    }
}

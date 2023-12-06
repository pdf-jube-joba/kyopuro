fn compute_part1(v: &Vec<(usize, usize)>) -> usize {
    let mut mul = 1;
    for &(time, distance) in v {
        let ways = (0..=time)
            .map(|t| t * (time - t))
            .filter(|dist| *dist > distance)
            .count();
        mul *= ways;
    }
    mul
}

fn concat(a: usize, mut b: usize) -> usize {
    let i: u32 = {
        let mut i = 1;
        while 10_usize.pow(i) <= b {
            i += 1;
        }
        i
    };
    a * 10_usize.pow(i) + b
}

fn compute_part2(v: &Vec<(usize, usize)>) -> usize {
    let mut time_sum = 0;
    let mut distance_sum = 0;
    for (time, distance) in v {
        time_sum = concat(time_sum, *time);
        distance_sum = concat(distance_sum, *distance);
    }
    println!("{time_sum} {distance_sum}");
    (0..=time_sum)
        .map(|t| t * (time_sum - t))
        .filter(|&dist| dist > distance_sum)
        .count()
}

fn main() {
    let v = input();
    println!("{}", compute_part1(&v));
    println!("{}", compute_part2(&v));
}

fn input() -> Vec<(usize, usize)> {
    let string = std::fs::read_to_string("inout/day6.in").unwrap();
    let mut lines = string.lines();
    let first_line = lines.next().unwrap();
    let vec1: Vec<usize> = {
        let i = first_line.find(":").unwrap() + 1;
        first_line[i..]
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect()
    };

    let second_line = lines.next().unwrap();
    let vec2: Vec<usize> = {
        let i = second_line.find(":").unwrap() + 1;
        second_line[i..]
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect()
    };
    vec1.into_iter().zip(vec2).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let v = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(compute_part1(&v), 288);
    }
    // b \in 0..10 => a * 10  + b
    #[test]
    fn concat_test() {
        // b \in 0..10 => a * 10  + b
        assert_eq!(concat(1, 0), 10);
        assert_eq!(concat(1, 1), 11);
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(1, 3), 13);

        assert_eq!(concat(1, 10), 110);
        assert_eq!(concat(1, 99), 199);
        assert_eq!(concat(12, 34), 1234);
        assert_eq!(concat(1, 1234), 11234);
    }
}

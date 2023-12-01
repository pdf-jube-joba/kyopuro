fn compute(str: &str) -> usize {
    str.lines().map(|line|{
        let v = line.chars().filter_map(|c| if c.is_ascii_digit() {
            Some(c.to_digit(10).unwrap() as usize)
        } else {
            None
        }).collect::<Vec<_>>();
        v[0] * 10 + v[v.len() - 1]
    }).sum()
}

fn main() {
    let content = input();
    let sum = compute(&content);
    println!("{sum}");
}

fn input() -> String {
    std::fs::read_to_string("inout/day1.in").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test() {
        let str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let sum = compute(str);
        assert_eq!(sum, 142);
    }
}
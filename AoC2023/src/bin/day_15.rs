fn main() {
    let input = input();
    println!("{}", compute_part2(input))
}

fn compute_part1(input: Vec<String>) -> usize {
    input.into_iter().map(|string| hash(&string)).sum()
}

fn compute_part2(input: Vec<String>) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 255];
    for string in input {
        if let Some(ind) = string.find('-') {
            let label = string[..ind].to_owned();
            let target_box = hash(&label);
            #[cfg(test)]
            println!("- case: {} {}", label, target_box);
            boxes[target_box].retain(|(elm, len)| *elm != label);
        } else {
            let ind = string.find('=').unwrap();
            let label = string[..ind].to_owned();
            let len: usize = string[ind + 1..].parse().unwrap();
            let target_box = hash(&label);
            #[cfg(test)]
            println!("= case {} {} {}", label, len, target_box);
            if let Some(ind) = boxes[target_box]
                .iter_mut()
                .find(|(label_in_box, len)| label == *label_in_box)
            {
                ind.1 = len;
            } else {
                boxes[target_box].push((label, len));
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, box_i)| {
            box_i
                .into_iter()
                .enumerate()
                .map(move |(j, lens)| (i + 1) * (j + 1) * lens.1)
        })
        .sum()
}

fn hash(string: &str) -> usize {
    let mut current_value = 0;
    for c in string.chars() {
        assert!(c.is_ascii());
        let ascii_code = c as usize;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn input() -> Vec<String> {
    let string = std::fs::read_to_string("inout/day15.in").unwrap();
    string.trim().split(',').map(|str| str.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_test() {
        let string = "HASH";
        assert_eq!(hash(&string), 52);

        let string = "rn=1";
        assert_eq!(hash(&string), 30);

        println!("{}", hash("rn"));
        println!("{}", hash("qp"));
        println!("{}", hash("cm"));
        println!("{}", hash("pc"));

        let string = "cm-";
        assert_eq!(hash(&string), 253);
    }
    #[test]
    fn part2_test() {
        let a: Vec<String> = vec![
            "rn=1",
            "cm-",
            "qp=3",
            "cm=2",
            "qp-",
            "pc=4",
            "ot=9",
            "ab=5",
            "pc-",
            "pc=6",
            "ot=7",
        ].into_iter().map(|str| str.to_owned()).collect();
        assert_eq!(compute_part2(a), 145);
    }
}

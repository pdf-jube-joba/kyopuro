#[derive(Debug, Clone, PartialEq)]
enum Sort {
    S,
    H,
    C,
    D,
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::S => "S".to_string(),
            Sort::H => "H".to_string(),
            Sort::C => "C".to_string(),
            Sort::D => "D".to_string(),
        }
    }
}

fn main() {
    let a = input();
    let mut a_bubble: Vec<(Sort, usize)> = a.clone();
    bubble(&mut a_bubble);
    print_vec(&a_bubble);
    if is_stable(&a, &a_bubble) {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    let mut a_select: Vec<(Sort, usize)> = a.clone();
    selection(&mut a_select);
    print_vec(&a_select);
    if is_stable(&a, &a_select) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}

fn print_vec(a: &[(Sort, usize)]) {
    for i in 0..a.len() {
        print!("{}{}", a[i].0.to_string(), a[i].1);
        if i != a.len() - 1 {
            print!(" ");
        }
    }
    println!()
}

fn is_stable(a_before: &[(Sort, usize)], a_after: &[(Sort, usize)]) -> bool {
    let n = a_after.len();
    let subseqs: Vec<&[(Sort, usize)]> = {
        let mut diff = vec![0];
        diff.extend((1..a_after.len()).filter(|i| a_after[*i].1 != a_after[*i - 1].1));
        diff.push(n);
        (0..diff.len() - 1)
            .map(|i| &a_after[diff[i]..diff[i + 1]])
            .collect()
    };
    for seq in subseqs {
        let mut i = 0;
        for card in seq {
            loop {
                if *card == a_before[i] {
                    break;
                }
                i += 1;
                if i == a_before.len() {
                    return false;
                }
            }
        }
    }
    true
}

fn bubble(a: &mut [(Sort, usize)]) {
    let n = a.len();
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if a[j].1 < a[j - 1].1 {
                a.swap(j, j - 1);
            }
        }
    }
}

fn selection(a: &mut [(Sort, usize)]) {
    let n = a.len();
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if a[j].1 < a[minj].1 {
                minj = j;
            }
        }
        a.swap(i, minj);
    }
}

fn input() -> Vec<(Sort, usize)> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
        .split_whitespace()
        .map(|str| {
            let sort: Sort = {
                match str.chars().nth(0) {
                    Some('S') => Sort::S,
                    Some('H') => Sort::H,
                    Some('C') => Sort::C,
                    Some('D') => Sort::D,
                    _ => panic!(),
                }
            };
            let num = str[1..].parse::<usize>().unwrap();
            (sort, num)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_test() {
        let before = vec![
            (Sort::S, 2),
            (Sort::H, 2),
        ];
        let afters = vec![
            (vec![
                (Sort::S, 2),
                (Sort::H, 2)
            ], true),
            (vec![
                (Sort::H, 2),
                (Sort::S, 2)
            ], false),
        ];
        for after in afters {
            assert_eq!(is_stable(&before, &after.0), after.1);
        }

        let before = vec![
            (Sort::S, 2),
            (Sort::H, 4),
            (Sort::H, 2),
            (Sort::D, 3),
        ];
        let afters = vec![
            (vec![
                (Sort::S, 2),
                (Sort::H, 2),
                (Sort::D, 3),
                (Sort::H, 4),
            ], true),
            (vec![
                (Sort::H, 2),
                (Sort::S, 2),
                (Sort::D, 3),
                (Sort::H, 4),
            ], false),
        ];
        for after in afters {
            assert_eq!(is_stable(&before, &after.0), after.1);
        }
    }
}

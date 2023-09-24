fn main() {
    let mut tarou_score: usize = 0;
    let mut hanako_score: usize = 0;
    let vec = input();
    for (tarou_card, hanako_card) in vec {
        let add_score = match compare_two_chars(&tarou_card, &hanako_card) {
            std::cmp::Ordering::Equal => (1,1),
            std::cmp::Ordering::Less => (0,3),
            std::cmp::Ordering::Greater => (3,0),
        };
        tarou_score += add_score.0;
        hanako_score += add_score.1;
    }
    println!("{} {}", tarou_score, hanako_score);
}

fn compare_two_chars(a: &[char], b: &[char]) -> std::cmp::Ordering {
    let max: usize = std::cmp::max(a.len(), b.len());
    for i in 0..max {
        let cmp = match (a.get(i), b.get(i)) {
            (Some(a_char), Some(b_char)) => compare_two_alphabet(*a_char, *b_char).unwrap(),
            (None, None) => std::cmp::Ordering::Equal,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
        };

        match cmp {
            std::cmp::Ordering::Equal => {
                continue;
            }
            _ => {
                return cmp;
            }
        }
    }
    std::cmp::Ordering::Equal
}

fn compare_two_alphabet(a: char, b: char) -> Option<std::cmp::Ordering> {
    match (char_to_alphabet_num(a), char_to_alphabet_num(b)) {
        (Some(a), Some(b)) => {
            Some(a.cmp(&b))
        },
        _ => None,
    }
}

// convert (a to 0), (b to 1), ..., (z to 25) and otherwise None
fn char_to_alphabet_num(char: char) -> Option<usize> {
    match char {
        'a' | 'A' => Some(0),
        'b' | 'B' => Some(1),
        'c' | 'C' => Some(2),
        'd' | 'D' => Some(3),
        'e' | 'E' => Some(4),
        'f' | 'F' => Some(5),
        'g' | 'G' => Some(6),
        'h' | 'H' => Some(7),
        'i' | 'I' => Some(8),
        'j' | 'J' => Some(9),
        'k' | 'K' => Some(10),
        'l' | 'L' => Some(11),
        'm' | 'M' => Some(12),
        'n' | 'N' => Some(13),
        'o' | 'O' => Some(14),
        'p' | 'P' => Some(15),
        'q' | 'Q' => Some(16),
        'r' | 'R' => Some(17),
        's' | 'S' => Some(18),
        't' | 'T' => Some(19),
        'u' | 'U' => Some(20),
        'v' | 'V' => Some(21),
        'w' | 'W' => Some(22),
        'x' | 'X' => Some(23),
        'y' | 'Y' => Some(24),
        'z' | 'Z' => Some(25),
        _ => None
    }
}

fn input() -> Vec<(Vec<char>, Vec<char>)> {
    let mut line_string = String::new();
    std::io::stdin().read_line(&mut line_string).unwrap();
    let n = line_string.trim().parse::<usize>().unwrap();

    (0..n).map(|_|{
        line_string = String::new();
        std::io::stdin().read_line(&mut line_string).unwrap();
        let v: Vec<&str> = line_string.split_whitespace().collect();
        (v[0].chars().collect(), v[1].chars().collect())
    }).collect()
}
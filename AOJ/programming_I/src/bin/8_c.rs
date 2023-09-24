fn main() {
    let input: String = input();
    let mut alphabet: Vec<usize> = vec![0; 26];
    input.chars().for_each(|char: char|
        if let Some(i) = char_to_alphabet_num(char) {
            alphabet[i] += 1;
        }
    );
    for i in 0..26 {
        println!("{} : {}", usize_to_alphavet(i).unwrap(), alphabet[i]);
    }
}

fn usize_to_alphavet(num: usize) -> Option<char> {
    match num {
        0  => Some('a'),
        1  => Some('b'),
        2  => Some('c'),
        3  => Some('d'),
        4  => Some('e'),
        5  => Some('f'),
        6  => Some('g'),
        7  => Some('h'),
        8  => Some('i'),
        9  => Some('j'),
        10 => Some('k'),
        11 => Some('l'),
        12 => Some('m'),
        13 => Some('n'),
        14 => Some('o'),
        15 => Some('p'),
        16 => Some('q'),
        17 => Some('r'),
        18 => Some('s'),
        19 => Some('t'),
        20 => Some('u'),
        21 => Some('v'),
        22 => Some('w'),
        23 => Some('x'),
        24 => Some('y'),
        25 => Some('z'),
        _ => None
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

fn input() -> String {
    use std::io::Read;
    let mut string = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_string(&mut string).unwrap();
    string
}
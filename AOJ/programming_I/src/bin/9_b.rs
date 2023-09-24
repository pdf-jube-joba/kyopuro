fn main() {
    while let Some((cards, _, v)) = input() {
        let shuffle: usize = v.iter().sum::<usize>() % cards.len();
        let car: String = cards.chars().take(shuffle).collect();
        let cdr: String = cards.chars().skip(shuffle).collect();
        println!("{}{}", cdr, car);
    }
}

fn input() -> Option<(String, usize, Vec<usize>)> {
    let stdin = std::io::stdin();
    let mut line_string = String::new();
    let cards: String = {
        stdin.read_line(&mut line_string).unwrap();
        line_string.trim().to_string()
    };
    if cards == "-" {
        return None;
    }

    let m: usize = {
        line_string = String::new();
        stdin.read_line(&mut line_string).unwrap();
        line_string.trim().parse::<usize>().unwrap()
    };

    let v: Vec<usize> = (0..m).map(|_|{
        line_string = String::new();
        stdin.read_line(&mut line_string).unwrap();
        line_string.trim().parse::<usize>().unwrap()
    }).collect();

    Some((cards, m, v))
}
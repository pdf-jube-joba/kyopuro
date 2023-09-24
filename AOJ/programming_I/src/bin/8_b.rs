fn main() {
    loop {
        let input: String = input();
        if input == "0\n" {
            break;
        }
        let sum: usize = input.chars().filter_map(|char: char|
            if char.is_whitespace() {
                None 
            } else {
                Some(char.to_string().parse::<usize>().unwrap())
            }
        ).sum();
        println!("{}", sum);
    }
}

fn input() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
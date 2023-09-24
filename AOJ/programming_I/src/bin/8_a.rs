fn main() {
    let line: String = {
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        string
    };
    let res: String = line.chars().map(|char: char| {
        if char.is_uppercase() {
            char.to_lowercase().collect::<String>()
        } else if char.is_lowercase() {
            char.to_uppercase().collect::<String>()
        } else {
            char.to_string()
        }
    }).collect::<String>();
    print!("{}", res);
}
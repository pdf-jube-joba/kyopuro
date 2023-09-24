fn main() {
    let (w, words) = input();
    let count: usize = words.into_iter().filter(|word| w == *word).count();
    println!("{}", count);
}

fn input() -> (String, Vec<String>) {
    use std::io::Read;
    let target_word: String = {
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        string.trim().to_string()
    };
    let mut string = String::new();
    std::io::stdin().read_to_string(&mut string).unwrap();
    let words: Vec<String> = string.split_whitespace().filter_map(|word| {
        if word != "END_OF_TEXT" {
            Some(word.to_owned().to_lowercase())
        } else {
            None
        }
    }).collect();

    (target_word, words)
}
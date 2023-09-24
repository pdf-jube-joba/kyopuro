fn main() {
    let (s, p) = input();

    let s_len = s.len();
    let p_len = p.len();

    let result = (0..s_len).any(|i|
        (0..p_len).all(|j|{
            let k = (i + j) % s_len;
            s[k] == p[j]
        })
    );
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input() -> (Vec<char>, Vec<char>) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let first: Vec<char> = string.chars().filter(|char| char.is_lowercase()).collect();
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let second: Vec<char> = string.chars().filter(|char| char.is_lowercase()).collect();
    (first, second)
}
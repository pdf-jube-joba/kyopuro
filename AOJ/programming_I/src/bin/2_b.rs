fn main() {
    let (a, b, c) = input();
    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input() -> (usize, usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1], vec[2])
}
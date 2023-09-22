fn main() {
    let (a, b) = input();
    let d = a / b;
    let r = a % b;
    let f = (a as f64) / (b as f64);
    println!("{} {} {}", d, r, f);
}

fn input() -> (usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1])
}
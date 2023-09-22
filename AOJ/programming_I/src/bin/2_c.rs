fn main() {
    let (n0, n1, n2) = input();
    let mut vec = vec![n0, n1, n2];
    vec.sort();
    println!("{} {} {}", vec[0], vec[1], vec[2]);
}

fn input() -> (usize, usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1], vec[2])
}
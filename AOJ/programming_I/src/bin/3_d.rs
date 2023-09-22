fn main() {
    let (a, b, c) = input();
    let num = (a..=b).filter(|i: &usize|{
        c % *i == 0
    }).count();
    println!("{}", num);
}

fn input() -> (usize, usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1], vec[2])
}
fn main() {
    loop {
        let (x, y) = input();
        if x == 0 && y == 0 {
            break;
        }
        if x < y {
            println!("{} {}", x, y);
        } else {
            println!("{} {}", y, x);
        }
    }
}

fn input() -> (usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1])
}
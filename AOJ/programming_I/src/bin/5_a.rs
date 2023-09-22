fn main() {
    loop {
        let (h, w) = input();
        if h == 0 && w == 0 {
            break
        }
        for _ in 0..h {
            for _ in 0..w {
                print!("#");
            }
            println!();
        }
        println!();
    }
}

fn input() -> (usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1])
}
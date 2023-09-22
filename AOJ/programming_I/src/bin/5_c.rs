fn main() {
    loop {
        let (h, w) = input();
        if h == 0 && w == 0 {
            break;
        }
        for i in 0..h {
            for j in 0..w {
                if (i + j) % 2 == 0 {
                    print!("#");
                } else {
                    print!(".");
                }
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
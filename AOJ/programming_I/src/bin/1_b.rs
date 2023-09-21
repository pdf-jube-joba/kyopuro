fn main() {
    let x: usize = input();
    println!("{}", x.pow(3));
}

fn input() -> usize {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string)
        .unwrap();
    string.trim().parse::<usize>()
        .unwrap()
}
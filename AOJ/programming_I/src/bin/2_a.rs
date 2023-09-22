fn main() {
    let (a, b) = input();

    if a < b {
        println!("a < b");
    } else if a > b {
        println!("a > b");
    } else {
        println!("a == b");
    }

}

fn input() -> (isize, isize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<isize>().unwrap()).collect();
    (vec[0], vec[1])
}
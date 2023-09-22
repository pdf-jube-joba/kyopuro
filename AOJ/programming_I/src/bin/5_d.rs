fn main() {
    let n = input();
    for i in 1..=n {
        if check_sum(i) || include_3(i) {
            print!(" {}", i);
        }
    }
}

fn check_sum(i: usize) -> bool {
    i % 3 == 0
}

fn include_3(i: usize) -> bool {
    let mut x = i;
    while x != 0 {
        if x % 10 == 3 {
            return true;
        }
        x /= 10;
    }
    false
}

fn input() -> usize {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<usize>().unwrap()).collect();
    vec[0]
}
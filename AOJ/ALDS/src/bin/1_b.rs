fn main() {
    let (x, y) = input();
    println!("{}", gcd(x, y));
}

fn gcd(x: usize, y: usize) -> usize {
    if x < y {
        return gcd(y, x);
    }
    // x >= y
    if x % y == 0 {
        y
    } else {
        gcd(y, x % y)
    }
}

fn input() -> (usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let v: Vec<_> =string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    (v[0], v[1])
}
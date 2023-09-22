fn main() {
    let r = input();
    let area = r * r * std::f64::consts::PI;
    let len = 2_f64 * std::f64::consts::PI * r;
    println!("{} {}", area, len);
}

fn input() -> f64 {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace()
        .map(|str| str.parse::<f64>().unwrap()).collect();
    vec[0]
}
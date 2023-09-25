fn main() {
    let (a, b, C) = input();
    let c = ( (a.powi(2)) + (b.powi(2)) - 2_f64 * a * b * C.cos() ).sqrt();
    let s: f64 = a * b * (C.sin()) * 0.5;
    let l: f64 = a + b + c;
    let h = b * (C.sin());
    println!("{}", s);
    println!("{}", l);
    println!("{}", h);
}

// C is radian
fn input() -> (f64, f64, f64) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let v: Vec<usize> = string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    (v[0] as f64, v[1] as f64, (v[2] as f64) / 180_f64 * std::f64::consts::PI)
}
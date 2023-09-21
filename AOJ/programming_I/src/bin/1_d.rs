fn main() {
    let total_s = input();
    let s = total_s % 60;
    let total_m = total_s / 60;
    let m = total_m % 60;
    let h = total_m / 60;
    println!("{}:{}:{}", h, m, s);
}

fn input() -> usize {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    str.trim().parse::<usize>().unwrap()
}
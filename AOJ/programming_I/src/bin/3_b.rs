fn main() {
    let mut i: usize = 1;
    let mut string;
    loop {
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let x = string.trim().parse::<usize>().unwrap();
        if x == 0 {
            break;
        }
        println!("Case {}: {}", i, x);
        i += 1;
    }
}
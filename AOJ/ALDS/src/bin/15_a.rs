fn main() {
    let mut rem = input();

    let num_25 = rem / 25;
    rem %= 25;
    let num_10 = rem / 10;
    rem %= 10;
    let num_5 = rem / 5;
    rem %= 5;
    let num_1 = rem;

    println!("{}", num_25 + num_10 + num_5 + num_1);

}

fn input() -> usize {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
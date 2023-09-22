fn main() {
    let (n, vec) = input();

    for i in 0..n {
        print!("{}", vec[n-i-1]);
        if i != n-1 {
            print!(" ");
        }
    }
    println!();
}

fn input() -> (usize, Vec<isize>) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let n = string.trim().parse::<usize>().unwrap();
    
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    
    let vec: Vec<isize> = string.split_whitespace().map(|str|{
        str.parse::<isize>().unwrap()
    }).collect();
    (n, vec)
}

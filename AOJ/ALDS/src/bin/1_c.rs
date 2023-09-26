fn main() {
    let numbers = input();
    let prime_num = numbers.into_iter().filter(|i| is_prime(*i)).count();
    println!("{}", prime_num);
}

fn is_prime(n: usize) -> bool {
    for i in 2_usize.. {
        if i.pow(2) > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut string).unwrap();
    let n = string.trim().parse::<usize>().unwrap();
    (0..n).map(|_|{
        string = String::new();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<usize>().unwrap()
    }).collect()
}
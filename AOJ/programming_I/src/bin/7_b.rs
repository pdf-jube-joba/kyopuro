fn main() {
    loop {
        let (n, x) = input();
        if n == 0 && x == 0 {
            break;
        }
        let mut count = 0;
        for i in 1..=n {
            for j in i+1..=n {
                if i + j < x {
                    let k = x - (i + j);
                    if j < k && k <= n {
                        count += 1;
                    }
                }
            }
        }
        println!("{}", count);
    }
}

fn input() -> (usize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<_> = string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    (vec[0], vec[1])
}
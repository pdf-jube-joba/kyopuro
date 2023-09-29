fn main() {
    let (s, t) = input();
    let num: usize = t.into_iter().filter(|v_t| {
        for v_s in &s {
            if *v_t == *v_s {
                return true;
            }
        }
        false
    }).count();
    println!("{}", num);
}

fn input() -> (Vec<usize>, Vec<usize>) {
    let _n = readline().trim().parse::<usize>().unwrap();
    let s: Vec<usize> = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    let _q = readline().trim().parse::<usize>().unwrap();
    let t: Vec<usize> = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    (s, t)
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
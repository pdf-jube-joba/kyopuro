fn main() {
    let (n, m, a, b) = input();
    for i in 0..n {
        let c_j: usize = a[i].iter().zip(&b).map(|(a_ij, b_j)| a_ij * b_j).sum();
        println!("{}", c_j);
    }
}

fn input() -> (usize, usize, Vec<Vec<usize>>, Vec<usize>) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    let nm_str: Vec<&str> = string.split_whitespace().collect();
    let n: usize = nm_str[0].parse().unwrap();
    let m: usize = nm_str[1].parse().unwrap();

    let mut a: Vec<_> = vec![];

    for _ in 0..n {
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let a_ij_row: Vec<usize> = string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
        a.push(a_ij_row);
    }

    let mut b: Vec<_> = vec![];

    for _ in 0..m {
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let b_row: usize = string.trim().parse().unwrap();
        b.push(b_row);
    }

    (n, m, a, b)
}
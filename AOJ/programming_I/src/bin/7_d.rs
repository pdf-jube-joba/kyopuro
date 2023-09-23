fn main() {
    let (n, m, l, a, b) = input();
    let mut vec = vec![vec![0; l]; n];

    for i in 0..n {
        for j in 0..l {
            vec[i][j] = (0..m).map(|k| a[i][k] * b[k][j]).sum()
        }
    }

    for i in 0..n {
        for j in 0..l {
            if j != 0 {
                print!(" ");
            }
            print!("{}", vec[i][j]);
        }
        println!();
    }

}

fn input() -> (usize, usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let v: Vec<_> = read_line().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    let (n, m, l) = (v[0], v[1], v[2]);

    let a: Vec<Vec<_>> = (0..n).map(|i| {
        read_line().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
    }).collect();
    let b: Vec<Vec<_>> = (0..m).map(|i| {
        read_line().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
    }).collect();
    (n, m, l, a, b)
}

fn read_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
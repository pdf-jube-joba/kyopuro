fn main() {
    let lists = input();
    let n = lists.len();
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in &lists[i] {
            matrix[i][*j] = 1;
        }
    }

    for row in matrix {
        println!("{}", row.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn input() -> Vec<Vec<usize>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let v: Vec<_> = buf
                .split_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            (&v[2..]).iter().map(|i| i-1).collect()
        })
        .collect::<Vec<_>>()
}

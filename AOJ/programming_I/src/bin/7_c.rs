fn main() {
    let (r, c, table) = input();
    let mut col_sum = vec![0; c];
    table.into_iter().for_each(|vec: Vec<usize>|{
        let sum: usize = vec.iter().sum();
        for (i, v) in vec.iter().enumerate() {
            col_sum[i] += v;
            print!("{} ", v);
        }
        println!("{}", sum);
    });
    for v in &col_sum {
        print!("{} ", v);
    }
    println!("{}", col_sum.iter().sum::<usize>());
}

fn input() -> (usize, usize, Vec<Vec<usize>>) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    let vec: Vec<usize> = string.split_whitespace().map(|str| str.parse().unwrap()).collect();

    let (r, c) = (vec[0], vec[1]);

    (
        r, 
        c, 
        (0..r).map(|_|{
            string = String::new();
            std::io::stdin().read_line(&mut string).unwrap();
            let vec: Vec<usize> = string.split_whitespace().map(|str| str.parse().unwrap()).collect();
            vec
        }).collect::<Vec<_>>()
    )
}
fn main() {
    let (p, q) = input();

    
}

fn input() -> (Vec<f64>, Vec<f64>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    let p = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|str| str.parse::<f64>().unwrap()).collect()
    };

    let q = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|str| str.parse::<f64>().unwrap()).collect()
    };

    (p, q)
}
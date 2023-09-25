fn main() {
    while let Some(vec) = input() {
        let n = vec.len();
        let m = vec.iter().sum::<f64>() / (n as f64);
        let sigma: f64 = (vec.iter().map(|si| (si - m).powi(2)).sum::<f64>() / (n as f64)).sqrt();
        println!("{}", sigma);
    }
}

fn input() -> Option<Vec<f64>> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    if string == "0\n" {
        return None;
    }
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    Some(string.split_whitespace().map(|str| str.parse::<usize>().unwrap() as f64).collect())
}
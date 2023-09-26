fn main() {
    let r = input();
    println!("{}", max_diff(r));
}

fn max_diff(r: Vec<i64>) -> i64 {
    let mut min: i64 = r[0];
    let mut diff_max = r[1] - r[0];
        for i in 1..r.len() {
            diff_max = std::cmp::max(diff_max, r[i] - min);
            min = std::cmp::min(min, r[i]);
        }
    diff_max
}

fn input() -> Vec<i64> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut string).unwrap();
    let n = string.trim().parse::<usize>().unwrap();
    (0..n).map(|_|{
        string = String::new();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<i64>().unwrap()
    }).collect()
}
fn main() {
    let vec = input();
    let mut min = std::isize::MAX;
    let mut max = std::isize::MIN;
    let mut sum = 0;
    for a_i in vec {
        if a_i < min {
            min = a_i;
        }
        if max < a_i {
            max = a_i;
        }
        sum += a_i;
    }
    println!("{} {} {}", min, max, sum);
}

fn input() -> Vec<isize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let _n = string.trim().parse::<usize>().unwrap();
    
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    
    let vec: Vec<isize> = string.split_whitespace().map(|str|{
        str.parse::<isize>().unwrap()
    }).collect();
    vec
}
fn main() {
    let mut a = input();
    let swap_count = selection(&mut a);
    print_vec(&a);
    println!("{}", swap_count);
}

fn print_vec(a: &[usize]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
        if i != a.len() - 1 {
            print!(" ");
        }
    }
    println!()
}

fn selection(a: &mut[usize]) -> usize {
    let n = a.len();
    let mut swap_count = 0;
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        if i != minj {
            a.swap(i, minj);
            swap_count += 1;
        }
    }
    swap_count
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
}
fn main() {
    let mut a = input();
    let swap_num = bubble(&mut a);
    print_vec(&a);
    println!("{}", swap_num);
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

fn bubble(a: &mut[usize]) -> usize {
    let n = a.len();
    let mut flag = true;
    let mut swap_count = 0;
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if a[j] < a[j-1] {
                a.swap(j, j-1);
                swap_count += 1;
                flag = true;
            }
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
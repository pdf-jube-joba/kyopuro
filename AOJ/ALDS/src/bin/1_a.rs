fn main() {
    let mut a = input();
    let n = a.len();
    for i in 1..n {
        let v = a[i];
        // j は <0 になることが想定されているアルゴリズムのようだ。
        // ただし、 a へのアクセス時には 0 以上であることが保たれる。
        let mut j: isize = (i - 1) as isize; 
        while  j >= 0 && a[j as usize] > v {
            a[(j+1) as usize] = a[j as usize];
            j -=1;
        }
        a[(j+1) as usize] = v;
        print(&a);
    }
}

fn print(a: &[usize]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
        if i != a.len() - 1 {
            print!(" ");
        }
    }
    println!()
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let _n = string.trim().parse::<usize>().unwrap();
    string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
}
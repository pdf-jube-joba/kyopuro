fn main() {
    let mut a = input();
    let v = partition(&mut a);
    for (i, ai) in a.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        if i == v {
            print!("[{}]", ai);
        } else {
            print!("{}", ai);
        }
    }
    println!()
}

fn partition(a: &mut [usize]) -> usize {
    let r = a.len() - 1;
    let x = a[r];
    let mut i: i32 = -1;
    for j in 0..r {
        if a[j] <= x {
            i += 1;
            a.swap(i as usize, j);
        }
    }
    a.swap((i + 1) as usize, r);
    (i + 1) as usize
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.clear();
    std::io::stdin().read_line(&mut string).unwrap();
    string
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect()
}


const K: usize = 10_000;

fn main() {
    let mut a = input();    
    let s = counting_sort(&a);
    for (i, si) in s.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", si);
    }
    println!()
}

fn counting_sort(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut b = vec![0; n];

    let mut c = vec![0; K + 1];

    for ai in a {
        c[*ai] += 1;
    }
    
    for i in 1..=K {
        c[i] += c[i-1];
    }

    for j in (0..n).rev() {
        let v = a[j];
        b[c[v] - 1] = v;
        c[a[j]] -= 1;
    }
    
    b
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.clear();
    std::io::stdin().read_line(&mut string).unwrap();
    string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
}
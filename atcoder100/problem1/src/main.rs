fn main() {

    let vec = input();
    for (n, x) in vec {
        println!("{}", count(n, x));
    }

}

fn input() -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let tuple: Vec<&str> = str
            .split_whitespace()
            .collect();
        let tuple: Vec<usize> = tuple
            .into_iter()
            .map(|str|{str.parse().unwrap()}).collect();
        let (x,y) = (tuple[0], tuple[1]);
        if x == 0 && y == 0 {
            break
        } else {
            vec.push((x, y));
        }
    }
    vec
}

fn count(n: usize, x: usize) -> usize {
    let mut count = 0;
    for i in 1..n {
        for j in i+1..n {
            let k = x - (i + j);
            if k > i && k > j && n >= k {
                count += 1;
            }
        }
    }
    count
}
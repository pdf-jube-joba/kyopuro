fn main() {
    let vec = input();
    let n = vec.len();
    let mut d = vec![None; n];
    let mut f = vec![None; n];
    depth_first_search(vec, &mut d, &mut f);
}

fn input() -> Vec<Vec<usize>> {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).expect("あ");
    let mut n: i32 = str.trim().parse().unwrap();
    let mut vec = Vec::new();
    while n != 0 {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("い");
        let v: Vec<usize> = s
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();
        vec.push(v[2..].to_owned());
        n -= 1;
    }
    vec
}

fn print(n: usize, d: &[Option<usize>], f: &[Option<usize>]) {
    let p = |n: Option<usize>| {
        if let Some(i) = n {
            i.to_string()
        } else {
            "*".to_string()
        }
    };
    for i in 0..n {
        println!("{} {} {}", i, p(d[i]), p(f[i]));
    }
}

fn depth_first_search(vec: Vec<Vec<usize>>, d: &mut [Option<usize>], f: &mut [Option<usize>]) {
    let mut l: Vec<usize> = Vec::new();
    let mut time = 1;
    let mut now = 0;
    loop {
        {
            println!("time {} now {} stack {:?}", time, now, l);
            print(vec.len(), d, f);
        }
        match (d[now], f[now]) {
            (None, None) => {
                d[now] = Some(time);
                time += 1;
                l.push(now);
            }
            (Some(_), None) => {
                if let Some(exist) = vec[now]
                    .clone()
                    .into_iter()
                    .find(|v| f[v.to_owned()].is_none())
                {
                    l.push(now);
                    now = exist;
                } else {
                    f[now] = Some(time);
                    time += 1;
                }
            }
            (Some(_), Some(_)) => {
                if let Some(next) = l.pop() {
                    now = next;
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let (s, t) = input();
    let num: usize = t.into_iter().filter(|&elem|{
        binary_search(&s, elem)
    }).count();
    println!("{}", num);
}

fn binary_search(sorted: &[usize], elem: usize) -> bool {
    let leq_i = |i: usize| sorted[i] <= elem;
    let mut ok: usize = if !leq_i(0) {
        return false;
    } else {
        0
    };
    let mut ng: usize = if leq_i(sorted.len() - 1) {
        return sorted[sorted.len() - 1] == elem;
    } else {
        sorted.len() - 1
    };
    while ng - ok > 1 {
        let mid = (ng - ok) / 2 + ok;
        if leq_i(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    assert!(ng - ok == 1);
    sorted[ok] == elem
}

fn input() -> (Vec<usize>, Vec<usize>) {
    let _n = readline().trim().parse::<usize>().unwrap();
    let s: Vec<usize> = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    let _q = readline().trim().parse::<usize>().unwrap();
    let t: Vec<usize> = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect();
    (s, t)
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: Usize1, x: usize, y: usize,
    }
    println!("{}", red((x, y), n))
}

fn red((x, y): (usize, usize), level: usize) -> usize {
    if level == 0 {
        return 0;
    }
    red((x, y), level - 1) + blue((x, y), level) * x
}

fn blue((x, y): (usize, usize), level: usize) -> usize {
    if level == 0 {
        return 1;
    }
    red((x, y), level - 1) + blue((x, y), level - 1) * y
}

fn main() {
    proconio::input! {
        n: usize,
    }
    let b = is_smooth(n);
    println!("{}", if b { "Yes" } else { "No" })
}

fn is_smooth(mut a: usize) -> bool {
    while a % 2 == 0 {
        a /= 2;
    }

    while a % 3 == 0 {
        a /= 3;
    }

    a == 1
}

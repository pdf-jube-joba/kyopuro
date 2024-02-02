fn main() {
    proconio::input! {
        mut n: usize,
    }
    let mut b = || -> bool {
        let mut prev = n % 10;
        n /= 10;
        while n != 0 {
            eprintln!("{prev} {n}");
            if n % 10 <= prev {
                return false;
            }
            prev = n % 10;
            n /= 10;
        }
        true
    };
    println!("{}", if b() { "Yes" } else { "No" })
}

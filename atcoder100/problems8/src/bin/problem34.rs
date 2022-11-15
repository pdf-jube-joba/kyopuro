fn main() {
    proconio::input! {
        n: usize,
    }
    println!("{}", fib(n));
}

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {return 1;}
    let mut x = 1;
    let mut y = 1;
    for _ in 0..n-1 {
        let z = x + y;
        x = y;
        y = z;
    }
    return y;
}

#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn fib_test(){
        assert_eq!(fib(0), 1);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 2);
        assert_eq!(fib(3), 3);
        assert_eq!(fib(4), 5);
        assert_eq!(fib(5), 8);
    }
}
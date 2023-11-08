fn fibonacchi(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut fib = Vec::with_capacity(n);
    fib.push(1);
    fib.push(1);
    for i in 2..=n {
        fib.push(fib[i-1] + fib[i-2]);
    }
    fib[n]
}

fn main() {
    let n = {
        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    println!("{}", fibonacchi(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fib_test() {
        assert_eq!(fibonacchi(0), 1);
        assert_eq!(fibonacchi(1), 1);
        assert_eq!(fibonacchi(2), 2);
        assert_eq!(fibonacchi(3), 3);
        assert_eq!(fibonacchi(4), 5);
        assert_eq!(fibonacchi(5), 8);
    }
}
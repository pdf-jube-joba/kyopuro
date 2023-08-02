fn main() {

    proconio::input!{
        a: usize, b: usize, c: usize, x: usize, y: usize,
    }

    println!("{}", value(a, b, c, x, y));

}

fn value(a: usize, b: usize, c: usize, x: usize, y: usize) -> usize {
    let f = |n|{
        let l = if x > n {x - n} else {0};
        let m = if y > n {y - n} else {0};
        a * l + b * m + c * (2 * n)
    };
    std::cmp::min(f(0), 
        std::cmp::min(f(x), f(y))
    )
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn value_test_1(){
        assert_eq!(value(1500, 2000, 1600, 3, 2), 7900);
    }
    #[test]
    fn value_test_2(){
        assert_eq!(value(1500, 2000, 1900, 3, 2), 8500);
    }
    #[test]
    fn value_test_3(){
        assert_eq!(value(1500, 2000, 500, 90_000, 100_000), 100_000_000)
    }
}
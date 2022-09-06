fn main() {

    proconio::input!{
        a: usize, b: usize, c: usize, x: usize, y: usize,
    }

    println!("{}", value(a, b, c, x, y));

}

fn value(a: usize, b: usize, c: usize, x: usize, y: usize) -> usize {
    if a + b > 2 * c {
        let c_num_to_value = |c_num|{
            let a_num = if x > (c_num / 2) {x - (c_num / 2)} else {0};
            let b_num = if y > (c_num / 2) {y - (c_num / 2)} else {0};
            a * a_num + b * b_num + c * c_num
        };
        let max = 2 * x + 2 * y;
        (0..max).map(c_num_to_value).min().unwrap()
    } else {
        x * a + y * b
    }
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
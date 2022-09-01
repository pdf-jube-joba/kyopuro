fn main() {

    proconio::input!{
        a: usize, b: usize, c: usize, x: usize, y: usize,
    }

    println!("{}", value(a, b, c, x, y));

}

fn value(a: usize, b: usize, c: usize, x: usize, y: usize) -> usize {
        if a + b > 2 * c {
            let m = std::cmp::min(x, y) / 2;
            let av = a * (x - (2 * m));
            let bv = b * (y - (2 * m));
            let cv = c * m;
            av + bv + cv
        } else {
            x * a + y * b
        }
}

#[test]
fn example1() {
    assert_eq!(value(1500, 2000, 1900, 3, 2), 8500)
}
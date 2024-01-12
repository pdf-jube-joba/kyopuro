use num_integer::Roots;

fn main() {
    proconio::input! {
        d: usize,
    }
    println!("{}", min_of_abs(d));
}

fn min_of_abs(d: usize) -> usize {
    (0..=d.sqrt()+1).map(|x|{
        let c = x.pow(2) as isize - d as isize;
        if c >= 0 {
            return c as usize;
        }
        let d1 = {
            let y = (-c).sqrt();
            c + (y.pow(2))
        };
        let d2 = {
            let y = (-c).sqrt() + 1;
            c + (y.pow(2))
        };
        std::cmp::min(d1.unsigned_abs(), d2.unsigned_abs())
    }).min().unwrap()
}

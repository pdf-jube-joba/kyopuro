fn main() {
    proconio::input! {
        n: usize,
    }

    println!("{}", count(n));

}

fn count(n: usize) -> usize {
    (1..=n).filter(|i| {
        i % 2 == 1 && (1..=*i).filter(|j|{i % j == 0}).count() == 8
    }).count()
}
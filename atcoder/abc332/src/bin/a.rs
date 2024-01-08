fn main() {
    proconio::input! {
        n: usize, s: usize, k: usize,
        pq: [(usize, usize); n],
    }
    println!("{}", cost(s, k, pq))
}

fn cost(s: usize, k: usize, pq: Vec<(usize, usize)>) -> usize {
    let sum: usize = pq.into_iter().map(|(p, q)| p * q).sum();
    if sum >= s {
        sum
    } else {
        sum + k
    }
}

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut a = vec![];
    for _ in 0..n {
        proconio::input! {
            li: usize,
            ai: [usize; li],
        }
        a.push(ai);
    }
    println!("{}", counting_array(a))
}

fn counting_array(mut a: Vec<Vec<usize>>) -> usize {
    a.sort();
    a.dedup();
    a.len()
}

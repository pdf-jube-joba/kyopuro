use num_integer::Roots;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", index_trio(a))
}

const M: usize = 200_000;

fn index_trio(a: Vec<usize>) -> usize {
    let mut num = vec![0; M + 1];
    for &ai in &a {
        num[ai] += 1;
    }
    a.into_iter()
        .map(|ai| {
            (1..=ai.sqrt())
                .filter(|y| ai % y == 0)
                .map(|y| num[y] * num[ai / y] * if y == ai / y { 1 } else { 2 })
                .sum::<usize>()
        })
        .sum()
}

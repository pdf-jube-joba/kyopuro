use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
    }
    let ans = many_balls(n);
    println!(
        "{}",
        ans.into_iter()
            .map(|m| match m {
                M::A => "A",
                M::B => "B",
            })
            .join("")
    )
}

#[derive(Debug, Clone)]
enum M {
    A,
    B,
}

fn many_balls(mut n: usize) -> Vec<M> {
    let mut v = vec![];
    while n != 0 {
        if n % 2 == 1 {
            v.push(M::A);
            n -= 1;
        } else {
            v.push(M::B);
            n /= 2;
        }
    }
    v.reverse();
    v
}

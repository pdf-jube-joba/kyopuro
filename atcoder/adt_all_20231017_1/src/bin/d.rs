use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
    }
    let mut xs: Vec<Vec<usize>> = vec![];
    for _ in 0..m {
        proconio::input! {
            k: usize, x: [Usize1; k],
        }
        xs.push(x);
    }

    println!("{}", if join_twice_all(n, xs) { "Yes" } else { "No" })
}

fn join_twice_all(n: usize, xs: Vec<Vec<usize>>) -> bool {
    for i in 0..n {
        for j in i + 1..n {
            if xs
                .iter()
                .all(|x| x.binary_search(&i).is_err() || x.binary_search(&j).is_err())
            {
                return false;
            }
        }
    }

    true
}

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        p: [Usize1; n-1],
    }
    let p: Vec<Option<usize>> = std::iter::once(None)
        .chain(p.into_iter().map(Some))
        .collect();
    println!("{}", ancestor_gen(p))
}

fn ancestor_gen(p: Vec<Option<usize>>) -> usize {
    let mut count = 0;
    let mut now = p.len() - 1;
    while let Some(p) = p[now] {
        count += 1;
        now = p;
    }
    count
}

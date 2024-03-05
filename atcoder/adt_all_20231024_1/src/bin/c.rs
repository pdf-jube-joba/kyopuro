fn main() {
    let cst: Vec<usize> = vec![3, 1, 4, 1, 5, 9];
    let f = |c: char| -> usize { c as usize - 'A' as usize };

    proconio::input! {
        p: char, q: char,
    }

    let (p, q) = (f(p), f(q));
    let (l, r) = (std::cmp::min(p, q), std::cmp::max(p, q));
    let dist = cst[l..r].iter().sum::<usize>();
    println!("{}", dist);
}

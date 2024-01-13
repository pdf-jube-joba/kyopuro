use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    for candidate in candidate(n, a) {
        println!("{}", candidate);
    }
}

fn candidate(n: usize, a: Vec<usize>) -> Vec<usize> {
    let m = a.len();
    let mut now_count = vec![0; n];
    let mut now_candidate = 0;
    (0..m).map(|i|{
        now_count[a[i]] += 1;
        if now_count[now_candidate] < now_count[a[i]] || (now_count[now_candidate] == now_count[a[i]] && a[i] < now_candidate) {
            now_candidate = a[i];
        }
        now_candidate + 1
    }).collect_vec()
}

use itertools::Itertools;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, q: usize,
    }
    let qs: Vec<Q> = (0..q)
        .map(|_| {
            proconio::input! {
                t: usize,
            }
            match t {
                1 => {
                    proconio::input! {
                        x: Usize1, y: Usize1,
                    }
                    Q::C(x, y)
                }
                2 => {
                    proconio::input! {
                        x: Usize1, y: Usize1,
                    }
                    Q::S(x, y)
                }
                3 => {
                    proconio::input! {
                        x: Usize1,
                    }
                    Q::P(x)
                }
                _ => unreachable!(),
            }
        })
        .collect();
    let ans = play_train(n, qs);
    for ans in ans {
        println!("{} {}", ans.len(), ans.into_iter().map(|i| i + 1).join(" "));
    }
}

enum Q {
    C(usize, usize),
    S(usize, usize),
    P(usize),
}

fn play_train(n: usize, qs: Vec<Q>) -> Vec<Vec<usize>> {
    let mut list: Vec<(Option<usize>, Option<usize>)> = vec![(None, None); n];
    let mut ans = vec![];
    for q in qs {
        match q {
            Q::C(x, y) => {
                list[x].1 = Some(y);
                list[y].0 = Some(x);
            }
            Q::S(x, y) => {
                list[x].1 = None;
                list[y].0 = None;
            }
            Q::P(x) => {
                let to_head = std::iter::successors(Some(x), |now| list[*now].0);
                let to_tail = std::iter::successors(Some(x), |now| list[*now].1);
                
                let mut ansq = vec![];
                ansq.extend(to_head);
                ansq.reverse();
                let _ = ansq.pop().unwrap();
                ansq.extend(to_tail);

                ans.push(ansq);
            }
        }
    }
    ans
}

use either::Either;
use proconio::marker::Chars;

const DIR: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn main() {
    proconio::input! {
        n: usize,
        t: Chars,
    }
    let t: Vec<Either<(), ()>> = t
        .into_iter()
        .map(|c| match c {
            'S' => Either::Left(()),
            'R' => Either::Right(()),
            _ => unreachable!(),
        })
        .collect();
    let ans = go_straight_and_turn_right(t);
    println!("{} {}", ans.0, ans.1);
}

fn go_straight_and_turn_right(t: Vec<Either<(), ()>>) -> (isize, isize) {
    let mut now = (0, 0);
    let mut now_dir = 0;
    for ti in t {
        match ti {
            Either::Left(()) => {
                now.0 += DIR[now_dir].0;
                now.1 += DIR[now_dir].1;
            }
            Either::Right(()) => {
                now_dir += 1;
                now_dir %= DIR.len();
            }
        }
    }
    now
}

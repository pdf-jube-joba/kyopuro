use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    }
    let s = s
        .into_iter()
        .map(|si| match si {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect_vec();
    println!("{}", robot_takahashi(s, w));
}

fn robot_takahashi(s: Vec<bool>, w: Vec<usize>) -> usize {
    let n = s.len();
    let mut sw = s.into_iter().zip(w.into_iter()).collect_vec();
    sw.sort_by_key(|(s, w)| *w);
    let mut judge = 0; // w[judge]
    let mut count = sw.iter().filter(|(s, w)| *s).count(); // X = w[judge] での正しく判定された人数
    let mut max = count;
    // count == f(w[judge])
    while judge < n {
        // X = w[judge + 1] での判定
        let same_right = sw[judge..n].partition_point(|&(s, w)| w == sw[judge].1);
        for j in judge..judge + same_right {
            if sw[j].0 {
                count -= 1;
            } else {
                count += 1;
            }
        }
        judge += same_right;
        max = std::cmp::max(max, count);
    }
    max
}

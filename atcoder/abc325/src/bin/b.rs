use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    println!("{}", max(wx))
}

fn max(wx: Vec<(usize, usize)>) -> usize {
    let n = wx.len();
    let w = wx.iter().map(|(w, x)| *w).collect_vec();
    let x = wx.iter().map(|(w, x)| *x).collect_vec();
    (0..24)
        .map(|time| {
            (0..n)
                .filter_map(|i| {
                    let time_at_i = (x[i] + time) % 24;
                    if 9 <= time_at_i && time_at_i + 1 <= 18 {
                        Some(w[i])
                    } else {
                        None
                    }
                })
                .sum()
        })
        .max()
        .unwrap()
}

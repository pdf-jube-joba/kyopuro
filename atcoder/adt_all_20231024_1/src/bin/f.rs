use itertools::iproduct;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: [Chars; 9],
    }
    let s: Vec<Vec<bool>> = s
        .into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    println!("{}", pown_square_num(s));
}

fn pown_square_num(s: Vec<Vec<bool>>) -> usize {
    let n = s.len();
    let to_range = |i: isize| -> Option<usize> {
        if 0 <= i && i < n as isize {
            Some(i as usize)
        } else {
            None
        }
    };

    let mut sq_collected: Vec<Vec<(usize, usize)>> =
        iproduct!(iproduct!(0..n, 0..n), iproduct!(0..n, 0..n))
            .filter(|&(p0, p1)| p0 != p1 && s[p0.0][p0.1] && s[p1.0][p1.1])
            .filter_map(|(p0, p1)| {
                let diff0: isize = p1.0 as isize - p0.0 as isize;
                let diff1: isize = p1.1 as isize - p0.1 as isize;

                let q0 = (p0.0 as isize + diff1, p0.1 as isize - diff0);
                let q1 = (p1.0 as isize + diff1, p1.1 as isize - diff0);
                let q0 = (to_range(q0.0)?, to_range(q0.1)?);
                let q1 = (to_range(q1.0)?, to_range(q1.1)?);

                if s[q0.0][q0.1] && s[q1.0][q1.1] {
                    Some(vec![p0, p1, q0, q1])
                } else {
                    None
                }
            })
            // .inspect(|v| eprintln!("{v:?}"))
            .collect();

    sq_collected.iter_mut().for_each(|v| v.sort());
    sq_collected.sort();
    sq_collected.dedup();
    sq_collected.len()
}

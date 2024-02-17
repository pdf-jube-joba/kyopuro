use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    let conv = |s: Vec<Vec<char>>| -> Vec<Vec<bool>> {
        s.into_iter()
            .map(|si| {
                si.into_iter()
                    .map(|char| match char {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    };
    let (s, t) = (conv(s), conv(t));
    println!(
        "{}",
        if is_equal_as_perm_rows(s, t) {
            "Yes"
        } else {
            "No"
        }
    )
}

fn transpose(a: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let (h, w) = (a.len(), a[0].len());
    let mut v = vec![vec![false; h]; w];
    for i in 0..h {
        for j in 0..w {
            v[j][i] = a[i][j]
        }
    }
    v
}

fn is_equal_as_perm_rows(s: Vec<Vec<bool>>, t: Vec<Vec<bool>>) -> bool {
    let (h, w) = (s.len(), s[0].len());
    // O(HW) to transpose
    let (mut s_trans, mut t_trans) = (transpose(&s), transpose(&t));
    // equality check => O(H) so sort is O(H W log W)
    s_trans.sort();
    t_trans.sort();
    s_trans == t_trans
}

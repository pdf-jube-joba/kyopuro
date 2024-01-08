use itertools::{iproduct, Itertools};

fn main() {
    proconio::input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }
    println!(
        "{}",
        if let Some(n) = min_of_op(a, b) {
            n as isize
        } else {
            -1
        }
    );
}

fn min_of_op(a: Vec<Vec<usize>>, b: Vec<Vec<usize>>) -> Option<usize> {
    let (h, w) = (a.len(), a[0].len());

    let rows_perms = (0..h).permutations(h);
    let cols_perms = (0..w).permutations(w);
    // h! * w! iteration
    iproduct!(rows_perms, cols_perms)
        .filter_map(|(rows_perm, cols_perm)| {
            // h * w to compute perm_a
            let perm_a = {
                let mut perm_a = vec![vec![0; w]; h];
                for ((i1, i2), (j1, j2)) in
                    iproduct!(rows_perm.iter().enumerate(), cols_perm.iter().enumerate())
                {
                    perm_a[i1][j1] = a[*i2][*j2];
                }
                perm_a
            };
            // check eq need h * w time
            if perm_a == b {
                // h^2 to compute step
                let row_step = step_perm(rows_perm.to_vec(), (0..h).collect::<Vec<_>>());
                // w^2 to compute step
                let col_step = step_perm(cols_perm.to_vec(), (0..w).collect::<Vec<_>>());
                Some(row_step + col_step)
            } else {
                None
            }
        })
        .min()
}

fn step_perm(perm1: Vec<usize>, perm2: Vec<usize>) -> usize {
    if perm1.len() == 1 {
        return 0;
    }
    let ind_2 = perm2.iter().position(|elm| *elm == perm1[0]).unwrap();
    let mut new_2 = vec![];
    new_2.extend(&perm2[0..ind_2]);
    new_2.extend(&perm2[ind_2 + 1..]);
    ind_2 + step_perm(perm1[1..].to_vec(), new_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn step_perm_test() {
        assert_eq!(step_perm(vec![1], vec![1]), 0);
        assert_eq!(step_perm(vec![1, 2], vec![1, 2]), 0);
        assert_eq!(step_perm(vec![1, 2], vec![2, 1]), 1);
        assert_eq!(step_perm(vec![2, 1], vec![1, 2]), 1);
        assert_eq!(step_perm(vec![1, 2, 3], vec![1, 2, 3]), 0);
        assert_eq!(step_perm(vec![1, 2, 3], vec![2, 1, 3]), 1);
        assert_eq!(step_perm(vec![1, 2, 3], vec![1, 3, 2]), 1);
        assert_eq!(step_perm(vec![1, 2, 3], vec![3, 2, 1]), 3);
        assert_eq!(step_perm(vec![1, 2, 3], vec![3, 1, 2]), 2);
        assert_eq!(step_perm(vec![1, 2, 3], vec![2, 3, 1]), 2);
    }
}

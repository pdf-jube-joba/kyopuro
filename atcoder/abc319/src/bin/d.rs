fn main() {
    proconio::input! {
        n: usize, m: usize,
        li: [usize; n],
    }
    println!("{}", min_window(li, m));
}

fn min_window(l: Vec<usize>, m: usize) -> usize {
    // minimal_linenum(l, ng) = None <=> cannot print in m line
    let mut ng = 0;
    // minimal_linenum(l, ok) = Some(1) <=> can print in n line and m line (because m <= n)
    let mut ok = l.iter().sum::<usize>() + (l.len() - 1);
    // assert!(minimal_linenum(&l, ok) == Some(1));
    // ok >= ng
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let res = minimal_linenum(&l, mid);
        // if ok
        match res {
            Some(l) if l <= m => ok = mid,
            _ => ng = mid,
        }
    }
    ok
}

// input: l: list of word's length, w: width of window
// output: min line number to print word in window
//  but return None if any of word satisfy length > window (it cant print on window)
fn minimal_linenum(l: &[usize], w: usize) -> Option<usize> {
    let mut line: Vec<Vec<usize>> = vec![vec![]];
    let mut current_col = 0;
    // current_col == (sum_i (ki + 1)) where [k0...ki] = line.last()
    // current_col <= w
    for &li in l {
        // word is too long
        if li > w {
            return None;
        }
        // go next line
        if current_col + li > w {
            line.push(vec![]);
            current_col = 0;
        }

        // put word and space
        line.last_mut().unwrap().push(li);
        current_col += li + 1;
    }
    Some(line.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_l() {
        let l = vec![1, 2, 3, 4];
        assert_eq!(minimal_linenum(&l, 1), None);
        assert_eq!(minimal_linenum(&l, 3), None);
        assert_eq!(minimal_linenum(&l, 4), Some(3));
        assert_eq!(minimal_linenum(&l, 5), Some(3));
        assert_eq!(minimal_linenum(&l, 6), Some(3));
        assert_eq!(minimal_linenum(&l, 7), Some(3));
        assert_eq!(minimal_linenum(&l, 8), Some(2));
        assert_eq!(minimal_linenum(&l, 13), Some(1));

        let l = vec![1,1,1,1,1,1];
        assert_eq!(minimal_linenum(&l, 1), Some(6));
        assert_eq!(minimal_linenum(&l, 2), Some(6));
        assert_eq!(minimal_linenum(&l, 3), Some(3));
        assert_eq!(minimal_linenum(&l, 4), Some(3));
        assert_eq!(minimal_linenum(&l, 5), Some(2));
        assert_eq!(minimal_linenum(&l, 6), Some(2));
        assert_eq!(minimal_linenum(&l, 7), Some(2));
        assert_eq!(minimal_linenum(&l, 8), Some(2));
        assert_eq!(minimal_linenum(&l, 11), Some(1));

        let w = min_window(l, 6);
        assert_eq!(w, 1)
    }
}

fn main() {
    proconio::input! {
        a: isize, m: usize, l: isize, r: isize,
    }
    println!("{}", compute(a, m, l, r));
}

fn compute(a: isize, m: usize, l: isize, r: isize) -> usize {
    // max k of a + km < l <=> km < l - a
    let left_most: isize = {
        let l = l - a;
        (l - 1).div_euclid(m as isize)
    };

    // max k of a + km <= r <=> km <= (r - a)
    let right_most: isize = {
        let r = r - a;
        r.div_euclid(m as isize)
    };

    if right_most < left_most {
        0
    } else {
        (right_most - left_most) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(compute(0, 1, -1, 1), 3);
        assert_eq!(compute(0, 1, 0, 1), 2);
        assert_eq!(compute(0, 1, 1, 2), 2);
        assert_eq!(compute(0, 1, -1, 0), 2);
        assert_eq!(compute(0, 2, 0, 0), 1);
        assert_eq!(compute(0, 2, 0, 4), 3);
        assert_eq!(compute(0, 2, 1, 3), 1);
        assert_eq!(compute(0, 2, 1, 4), 2);
        assert_eq!(compute(0, 2, 2, 4), 2);
        assert_eq!(compute(0, 2, 3, 4), 1);
        assert_eq!(compute(0, 2, -1, 1), 1);
        assert_eq!(compute(0, 2, -1, 2), 2);
        assert_eq!(compute(0, 2, -1, 0), 1);
        assert_eq!(compute(0, 2, -2, 0), 2);
    }
}

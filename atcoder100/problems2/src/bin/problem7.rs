use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: usize,
        a: [(i32, i32); n],
    }

    println!("{}", output(&a));
}

fn output(p: &[(i32, i32)]) -> usize {
    let p: HashSet<(i32, i32)> = HashSet::from_iter(p.iter().cloned());
    p.iter()
        .flat_map(|i| p.iter().map(move |j| (i, j)))
        .filter_map(|(&x1, &x2)| {
            let v: (i32, i32) = (x2.1 - x1.1, -(x2.0 - x1.0));
            let x3: (i32, i32) = (x1.0 + v.0, x1.1 + v.1);
            let x4: (i32, i32) = (x2.0 + v.0, x2.1 + v.1);
            if p.contains(&x3) && p.contains(&x4) {
                Some(((x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2)) as usize)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_output_1() {
        assert_eq!(
            output(&[
                (9, 4),
                (4, 3),
                (1, 1),
                (4, 2),
                (2, 4),
                (5, 8),
                (4, 0),
                (5, 3),
                (0, 5),
                (5, 2)
            ]),
            10
        )
    }
}

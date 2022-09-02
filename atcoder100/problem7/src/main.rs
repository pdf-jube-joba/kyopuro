fn main() {
    proconio::input! {
        n: usize,
        a: [(i32, i32); n],
    }

    println!("{}", output(&a));

}

fn output(a: &[(i32, i32)]) -> usize {
    let mut max = 0;
    for x1 in a {
        for x2 in a {
            let v: (i32, i32) = (x2.1 - x1.1, - (x2.0 - x1.0));
            let x3: (i32, i32) = (x1.0 + v.0, x1.1 + v.1);
            let x4: (i32, i32) = (x2.0 + v.0, x2.1 + v.1);
            let result =
                a.into_iter().any(|y|{y.0 == x3.0 && y.1 == x3.1}) &&
                a.into_iter().any(|y|{y.0 == x4.0 && y.1 == x4.1});
            let area = (x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2);
            if result {
                max = std::cmp::max(max, area);
            }
        }
    }
    max.try_into().unwrap()
}

#[test]
fn example() {
    assert_eq!(output(&vec![(9,4), (4,3), (1,1), (4,2), (2,4), (5,8), (4,0), (5,3), (0,5), (5,2)]), 10)
}
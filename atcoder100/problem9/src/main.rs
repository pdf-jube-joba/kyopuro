use std::result;

fn main() {
    proconio::input! {
        m: usize,
        ms: [(i32, i32); m],
        n: usize,
        ns: [(i32, i32); n],
    }

    let result = vec_par(&ms, &ns);
    println!("{} {}", result.0, result.1);

}

fn vec_par(ms: &[(i32, i32)], ns: &[(i32, i32)]) -> (i32, i32) {
    for x in ns {
        let vec = (x.0 - ms[0].0 , x.1 - ms[0].1);
        let ms2: Vec<(i32, i32)> = ms.iter().map(|y|{
            (y.0 + vec.0, y.1 + vec.1)
        }).collect();
        println!("{:?}", ms2);
        if ms2.into_iter().all(|y|{
            ns.into_iter().any(|z|{y.0 == z.0 && y.1 == z.1})
        }) {
            return vec;
        }
    }
    unreachable!()
}

#[test]
fn example () {
    let ns = vec![(8,5), (6,4), (4,3), (7,10), (0,10)];
    let ms = vec![(10,5), (2,7), (9,7), (8,10), (10,2), (1,2), (8,1), (6,7), (6,0), (0,9)];
    assert_eq!(vec_par(&ns, &ms), (2, -3));
}

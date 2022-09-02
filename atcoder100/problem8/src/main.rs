fn main() {
    proconio::input! {
        n: usize,
        a: [(usize, usize); n],
    }

    println!("{}", min_of_1(&a));

}

fn abs_m_u(x: usize, y: usize) -> usize {
    if x > y { x - y } else { y - x }
}

fn min_of_1(a: &[(usize, usize)]) -> usize {
    let mut min = std::usize::MAX;
    for s in 1..=100 {
        for t in 1..=100 {
            let mut sum = 0;
            for n in a {
                sum += abs_m_u(s, n.0) + abs_m_u(n.0, n.1) + abs_m_u(n.1, t);
            }
            if sum < min {min = sum;}
        }
    }
    min
}

fn min_of_2(a: &[(usize, usize)]) -> usize {
    let s_min = {
        let mut vec: Vec<usize> = a.into_iter()
            .map(|(x,_)|{*x}).clone().collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().fold(0, |acc, item|{acc + abs_m_u(item, mid)})
    };
    let m_cst = {
        let mut m = 0;
        a.into_iter().for_each(|(x,y)|{m += abs_m_u(*x, *y)});
        m
    };
    let t_min = {
        let mut vec: Vec<usize> = a.into_iter()
            .map(|(_, y)|{*y}).clone().collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().fold(0, |acc, item|{acc + abs_m_u(item, mid)})
    };
    s_min + m_cst + t_min
}

#[test]
fn example () {
    assert_eq!(min_of_1(&vec![(5,7), (2,6), (8,10)]), 18);
    assert_eq!(min_of_2(&vec![(5,7), (2,6), (8,10)]), 18);
}

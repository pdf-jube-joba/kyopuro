use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize, k: usize,
        in1: [(usize, usize); k]
    }
    let s: Vec<(usize, Source)> = in1.iter()
        .map(|&(n,m)|(n, Source::from_num(m).unwrap())).collect();
    
    println!("{}", count(n, &s));
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Source {
    Tomato,
    Cream,
    Basil,
}

impl Source {
    fn from_num(s: usize) -> Option<Source> {
        match s {
            1 => Some(Source::Tomato),
            2 => Some(Source::Cream),
            3 => Some(Source::Basil),
            _ => None,
        }
    }
}

fn count(n: usize, s: &[(usize, Source)]) -> usize {
    let all_pairs = vec![
        (Source::Tomato, Source::Tomato),
        (Source::Tomato, Source::Cream),
        (Source::Tomato, Source::Basil),
        (Source::Cream, Source::Tomato),
        (Source::Cream, Source::Cream),
        (Source::Cream, Source::Basil),
        (Source::Basil, Source::Tomato),
        (Source::Basil, Source::Cream),
        (Source::Basil, Source::Basil),
    ];

    // default[i] は i 日目が決まっているならその内容、そうでないなら None を返す
    // 0-index に変換する
    let default: HashMap<usize, Source> = HashMap::from_iter(s.iter().cloned().map(|(n, s)| (n-1, s)));
    // また、 n についても 0-index に変換する
    let n = n-1;
    
    // dp[A][B][i] は 「i 日目が A」で「i+1 日目が B」となり「すでに定まった予定に合致した」予定が何通りあるか
    let mut dp: HashMap<(Source, Source), Vec<usize>> = HashMap::new();

    for &pair in &all_pairs {
        dp.insert(pair, Vec::with_capacity(n-1));
    }

    for &pair in &all_pairs {
        dp.get_mut(&pair).unwrap().push({
            if (match default.get(&0) {
                Some(&already) => already != pair.0,
                _ => false,
            }) || ( match default.get(&1) {
                Some(&already) => already != pair.1,
                _ => false,
            }) {0} else {1}
        });
    }

    for i in 0..n-1 {
        for &pair in &all_pairs {
            let result = {
                let match_now = match default.get(&(i+1)) {
                    Some(&already) => already != pair.0,
                    _ => false,
                };
                let match_next = match default.get(&(i+2)) {
                    Some(&already) => already != pair.1,
                    _ => false,
                };
                if match_now || match_next {0} else {
                    vec![Source::Tomato, Source::Cream, Source::Basil].iter()
                        .map(|&prev|{
                            println!("{} {:?} {:?}", i, prev, pair);
                            if prev == pair.0 && pair.0 == pair.1 {0}
                            else {dp.get(&(prev, pair.0)).unwrap()[i]}
                        }).sum()
                }
            } % 10_000 ;
            dp.get_mut(&pair).unwrap().push(result);
        }
    }

    println!("{:?}", default);
    println!("{:?}", dp);

    all_pairs.iter().map(|pair|{
        dp.get(pair).unwrap().last().unwrap()
    }).sum::<usize>()  % 10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test_1(){
        let n = 5;
        let s: Vec<(usize, Source)> = vec![(3,1), (1,1), (4,2)]
            .iter().map(|&(n,s)| (n, Source::from_num(s).unwrap())).collect();
        assert_eq!(count(n, &s), 6);
    }
    #[test]
    fn count_test_o1(){
        let n = 3;
        vec![
            vec![(1,1), (2,1)],
            vec![(1,1), (3,1)],
            vec![(2,1), (3,1)],
        ].into_iter().for_each(|vec|{
            let s: Vec<(usize, Source)> = vec.iter().map(|&(n,s)| (n, Source::from_num(s).unwrap())).collect();
            assert_eq!(count(n, &s), 2);
        });
    }
    #[test]
    fn count_test_o2(){
        let n = 3;
        vec![
            vec![(1,1), (2,2)],
            vec![(1,1), (3,2)],
            vec![(2,1), (3,2)],
        ].into_iter().for_each(|vec|{
            let s: Vec<(usize, Source)> = vec.iter().map(|&(n,s)| (n, Source::from_num(s).unwrap())).collect();
            assert_eq!(count(n, &s), 3);
        });
    }
    #[test]
    fn count_test_o3(){
        assert_eq!(count(3, &Vec::new()), 27-3)
    }
    #[test]
    fn count_test_2(){
        let n = 20;
        let s: Vec<(usize, Source)> = vec![(10,2), (4,3), (12,1), (13,2), (9,1)]
            .iter().map(|&(n,s)| (n, Source::from_num(s).unwrap())).collect();
        assert_eq!(count(n, &s), 2640);
    }
}
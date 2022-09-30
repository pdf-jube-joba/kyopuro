use std::collections::VecDeque;

fn to_bool(v: &[usize]) -> Vec<bool> {
    v.iter()
    .map(|i| match *i {0 => true, 1 => false, _ => unreachable!()})
    .collect()
}

fn print_map(m: &[Vec<bool>]) {
    (0..m.len()).map(|i|{
        if i % 2 == 0 {
            let mut str = String::new();
            m[i].iter().for_each(|&b| str.push_str(if b {"0 "} else {"1 "}));
            str
        } else {
            let mut str = String::new();
            m[i].iter().for_each(|&b| str.push_str(if b {" 0"} else {" 1"}));
            str
        }
    }).for_each(|str|{println!("{}", str);});
}

fn input() -> Vec<Vec<bool>> {
    proconio::input! {
        w: usize, h: usize,
        b: [[usize; w]; h],
    }
    b.iter().map(|b|{
        to_bool(b)
    }).collect()
}

fn main() {
    let b = input();
    println!("{}", total(&b));
}

fn total(b: &[Vec<bool>]) -> usize {
    let h = b.len();
    let w = b[0].len();
    let b = {
        let mut v = vec![vec![true; w + 2]; h + 2];
        for i in 0..h {
            for j in 0..w {
                v[i + 1][j + 1] = b[i][j]; 
            }
        }
        v
    };

    let surr = |(i,j): (usize, usize)|{
        if i % 2 == 0 {
            vec![
                if 0 < i && 0 < j {Some((i-1, j-1))} else {None}, // 北西
                if 0 < i {Some((i-1, j))} else {None}, // 北東
                if 0 < j {Some((i, j-1))} else {None}, // 西
                if j < w+1 {Some((i, j+1))} else {None}, // 東
                if i < h+1 && 0 < j {Some((i+1, j-1))} else {None}, // 南西
                if i < h+1 {Some((i+1, j))} else {None}, // 南東
            ]
        } else {
            vec![
                if 0 < i {Some((i-1, j))} else {None}, // 北西
                if 0 < i && j < w+1 {Some((i-1, j+1))} else {None}, // 北東
                if 0 < j {Some((i, j-1))} else {None}, // 西
                if j < w+1 {Some((i, j+1))} else {None}, // 東
                if i < h+1 {Some((i+1, j))} else {None}, // 南西
                if i < h+1 && j < w+1 {Some((i+1, j+1))} else {None}, // 南東
            ]
        }
        .into_iter().flatten().collect::<Vec<(usize, usize)>>()
    };

    let mut memo = vec![vec![false; w + 2]; h + 2];
    let mut queue = VecDeque::new();
    queue.push_back((0,0));
    memo[0][0] = true;
    while let Some(now) = queue.pop_front() {
        surr(now).into_iter().for_each(|(i,j)|{
            println!("a {:?}", (i,j));
            if (!memo[i][j]) && b[i][j] {
                memo[i][j] = true;
                queue.push_back((i,j));
            }
        });
    }

    (0..h+2).flat_map(|i| (0..w+2).map(move |j| (i,j)))
    .filter(|&(i,j)| memo[i][j])
    .map(|(i,j)|{
        surr((i,j)).into_iter().filter(|&(i,j)| !memo[i][j]).count()
    })
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_test(){
        let b = vec![
            to_bool(&vec![0, 1, 0, 1, 0, 1, 1, 1]),
            to_bool(&vec![0, 1, 1, 0, 0, 1, 0, 0]),
            to_bool(&vec![1, 0, 1, 0, 1, 1, 1, 1]),
            to_bool(&vec![0, 1, 1, 0, 1, 0, 1, 0]),
        ];
        let result = total(&b);
        assert_eq!(result, 64);
    }
}
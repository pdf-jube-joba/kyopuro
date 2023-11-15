fn abs_diff(i: usize, j: usize) -> usize {
    if i < j { j - i } else { i - j }
}

fn is_valid(queens: &[(usize, usize)], queens_alloc: &[usize]) -> bool {
    let q = combine(queens, queens_alloc);

    for i in 0..8 {
        for j in i+1..8 {
            match (q[i], q[j]) {
                (Some(qi), Some(qj)) => {
                    if qi == qj || abs_diff(qi, qj) == abs_diff(i, j) {
                        return false;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    true
    
}

fn combine(queens: &[(usize, usize)], queens_alloc: &[usize]) -> Vec<Option<usize>> {
    let mut j = 0;
    (0..8).map(|i|{
        if let Some(p) = queens.iter().find_map(|(j, p)| {
            if i == *j { Some(p) } else { None }
        }) {
            Some(*p)
        } else if j < queens_alloc.len() {
            let t = queens_alloc[j];
            j += 1;
            Some(t)
        } else {
            None
        }
    }).collect::<Vec<_>>()
}

fn queens_al(queens: Vec<(usize, usize)>) -> Vec<usize> {
    let mut queens_alloc = vec![];
    loop {
        if is_valid(&queens, &queens_alloc) {
            if queens.len() + queens_alloc.len() == 8 {
                break;
            } else {
                queens_alloc.push(0);
            }
        } else {
            while queens_alloc[queens_alloc.len() - 1] == 7 {
                queens_alloc.pop();
            }
            let i = queens_alloc.pop().unwrap();
            queens_alloc.push(i + 1);
        }
    }
    queens_alloc
}

fn print_str(q: &[Option<usize>]) -> String {
    let mut s = String::new();
    for i in 0..8 {
        if let Some(j) = q[i] {
            s.push_str(&format!("{}\n",{
                (0..8).map(|k| if j == k {'Q'} else {'.'}).collect::<String>()
            }));
        } else {
            s.push_str("........\n")
        }
    }
    s
}

fn main() {
    let queens = input();
    let queens_alloc = queens_al(queens.clone());
    let q = combine(&queens, &queens_alloc);
    print!("{}", print_str(&q));
}

fn input() -> Vec<(usize, usize)> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let k = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..k).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let v = buf.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (v[0], v[1])
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combine_test() {
        let queens = vec![];
        let queens_alloc = vec![];
        let expect = vec![None, None, None, None, None, None, None, None,];
        assert_eq!(combine(&queens, &queens_alloc), expect);

        let queens = vec![(2,3), (5,6)];
        let queens_alloc = vec![];
        let expect = vec![None, None, Some(3), None, None, Some(6), None, None,];
        assert_eq!(combine(&queens, &queens_alloc), expect);

        let queens = vec![];
        let queens_alloc = vec![3,4,5];
        let expect = vec![Some(3), Some(4), Some(5), None, None, None, None, None,];
        assert_eq!(combine(&queens, &queens_alloc), expect);

        let queens = vec![(3,6), (4,0)];
        let queens_alloc = vec![3,4,5];
        let expect = vec![Some(3), Some(4), Some(5), Some(6), Some(0), None, None, None,];
        assert_eq!(combine(&queens, &queens_alloc), expect);

        let queens = vec![(3,6), (4,0), (7,1)];
        let queens_alloc = vec![3,4,5,2,7];
        let expect = vec![Some(3), Some(4), Some(5), Some(6), Some(0), Some(2), Some(7), Some(1),];
        assert_eq!(combine(&queens, &queens_alloc), expect);
    }
    #[test]
    fn valid_test() {
        let queens = vec![];
        let queens_alloc = vec![];
        assert!(is_valid(&queens, &queens_alloc));

        let queens = vec![];
        let queens_alloc = vec![1];
        assert!(is_valid(&queens, &queens_alloc));

        let queens = vec![(0, 0)];
        let queens_alloc = vec![0];
        assert!(!is_valid(&queens, &queens_alloc));

        let queens = vec![(0, 0)];
        let queens_alloc = vec![1];
        assert!(!is_valid(&queens, &queens_alloc));

        let queens = vec![(0, 0)];
        let queens_alloc = vec![2];
        assert!(is_valid(&queens, &queens_alloc));

        let queens = vec![(0, 0), (1, 2)];
        let queens_alloc = vec![4,6];
        assert!(is_valid(&queens, &queens_alloc));
    }
}
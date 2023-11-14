fn abs_diff(i: usize, j: usize) -> usize {
    if i < j { j - i } else { i - j }
}

fn is_valid(queens: &[(usize, usize)], queens_alloc: &[usize]) -> bool {
    let mut j = 0;
    let q = (0..8).map(|i|{
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
    }).collect::<Vec<_>>();

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

fn queens(queens: Vec<(usize, usize)>) -> Vec<usize> {
    let mut queens_alloc = vec![];
    loop {
        if is_valid(&queens, &queens_alloc) {
            if queens.len() + queens_alloc.len() == 8 {
                break;
            } else {
                queens_alloc.push(0);
            }
        } else {
            if queens_alloc[queens_alloc.len() - 1] == 7 {
                queens_alloc.pop();
            }
            let i = queens_alloc.pop().unwrap();
            queens_alloc.push(i + 1);
        }
    }
    queens_alloc
}

fn main() {
    let queens = input();
    
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
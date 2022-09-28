fn input() -> Vec<Vec<usize>> {
    proconio::input! {
        n: usize,
    }
    let mut vec = vec![Vec::new();n];
    let mut lines = std::io::stdin().lines();
    while let Some(Ok(str)) = lines.next() {
        let mut strs = str.split_whitespace();
        let _n: usize = strs.next().unwrap().parse().unwrap();
        let _k: usize = strs.next().unwrap().parse().unwrap();
        vec.push(strs.map(|str|{str.parse::<usize>().unwrap() - 1}).collect());
    }
    vec
}

fn main() {
    let vec = input();
    let result = BFS(&vec);
    for i in 0..result.len(){
        println!("{} {}", i, match result[i] {Some(num) => num as i32, None => -1});
    }
}

fn BFS(vec: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = vec.len();
    let mut result: Vec<Option<usize>> = vec![None;n];
    let mut queue = std::collections::VecDeque::new();
    queue.push_front(0);
    result[0] = Some(0);
    while let Some(now) = queue.pop_front() {
        let len = result[now].unwrap();
        for &next in &vec[now] {
            if result[next].is_none() {
                result[next] = Some(len + 1);
                queue.push_back(next);
            }
        }
    }
    result
}

fn BFS_rec(now: usize, vec: &[Vec<usize>], rec: &mut [Option<usize>]) {
    let len = rec[now].unwrap();
    for &next in &vec[now] {
        if rec[next].is_none() {
            rec[next] = Some(len + 1);
            BFS_rec(next, vec, rec);
        }
    }
}

fn BFS2(vec: &[Vec<usize>]) -> Vec<Option<usize>> {
    let mut rec = vec![None; vec.len()];
    rec[1] = Some(0);
    BFS_rec(1, vec, &mut rec);
    rec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_BFS_1(){
        let vec = vec![
            vec![1,3],
            vec![3],
            vec![],
            vec![2],
        ];
        let expect = vec![
            Some(0),
            Some(1),
            Some(2),
            Some(1),
        ];
        let result = BFS(&vec);
        assert_eq!(result, expect);
    }
    #[test]
    fn test_BFS_2(){
        let vec = vec![
            vec![1,3],
            vec![4],
            vec![4,5],
            vec![],
            vec![3],
            vec![5],
        ];
        let expect = vec![
            Some(0),
            Some(1),
            None,
            Some(1),
            Some(2),
            None,
        ];
        let result = BFS(&vec);
        assert_eq!(result, expect);
    }
}
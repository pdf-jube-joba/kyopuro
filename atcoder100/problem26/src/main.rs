use std::vec;

fn main() {
    proconio::input! {
        n: usize, q:usize,
        ab: [(usize, usize); n-1],
        px: [(usize, usize); q],
    }

    let result = solve(n, &ab, &px);
    (0..n).for_each(|i|{
        print!("{} ", result[i]);
    });
}

fn solve(n: usize, ab: &[(usize, usize)], px: &[(usize, usize)]) -> Vec<usize> {
    fn dfs(
        now: usize,
        parent: Option<usize>,
        point: &[usize],
        tree: &[Vec<usize>],
        dp: &mut Vec<usize>,
        acc: usize,
    ) {
        let next = point[now] + acc;
        dp[now] = next;
        for &node in &tree[now] {
            if Some(node) != parent {
                dfs(node, Some(now), point, tree, dp, next);
            }
        }
    }

    let point = {
        let mut vec = vec![0; n];
        for &(a, b) in px {
            vec[a - 1] += b;
        }
        vec
    };
    let tree = {
        let mut vec = vec![Vec::new(); n];
        for &(a, b) in ab {
            vec[a - 1].push(b - 1);
            vec[b - 1].push(a - 1);
        }
        vec
    };
    let mut dp = vec![0; n];
    dfs(0, None, &point, &tree, &mut dp, 0);
    dp
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use crate::*;
    #[test]
    fn test_1(){
        let n = 4;
        let _q = 3;
        let ab = vec![(1,2),(2,3),(2,4)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result[1..], vec![100,110,111,110]);
    }
    #[test]
    fn test_2(){
        let n = 3;
        let _q = 3;
        let ab = vec![(1,3),(2,3)];
        let px = vec![(2,10),(1,100),(3,1)];
        let result = solve(n, &ab, &px);
        assert_eq!(result[1..], vec![100,111,101]);
    }
    #[test]
    fn test_3(){
        let in_file_name = "./in/b04";
        let out_file_name = "./out/b04";
        let (n, _q, ab, px) = {
            let file = std::fs::File::open(in_file_name).unwrap();
            let mut lines = std::io::BufReader::new(file).lines();
            let nums: Vec<usize> = lines.next().unwrap().unwrap()
                .split_whitespace().map(|s|s.parse().unwrap()).collect();
            let n = nums[0];
            let q = nums[1];
            let ab: Vec<(usize, usize)> = {
                (0..n-1).map(|_|{
                    let nums: Vec<usize> = lines.next().unwrap().unwrap()
                    .split_whitespace().map(|s|s.parse().unwrap()).collect();
                    (nums[0], nums[1])
                }).collect()
            };
            let px: Vec<(usize, usize)> = {
                (0..q).map(|_|{
                    let nums: Vec<usize> = lines.next().unwrap().unwrap()
                    .split_whitespace().map(|s|s.parse().unwrap()).collect();
                    (nums[0], nums[1])
                }).collect()
            };
            (n, q, ab, px)
        };
        let expect: Vec<usize> = {
            let file = std::fs::File::open(out_file_name).unwrap();
            let lines = std::io::BufReader::new(file).lines();
            lines.flat_map(|str|{
                str.unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<usize>>()
            }).collect()
        };
        let result = solve(n, &ab, &px);
        assert_eq!(result, expect);
    }
}
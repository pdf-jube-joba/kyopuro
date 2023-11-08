fn longest_common_subseq<T: PartialEq + Clone>(x: &[T], y: &[T]) -> usize {
    let lx = x.len();
    let ly = y.len();
    if lx == 0 || ly == 0 {
        return 0;
    }

    // dp[i][j] = len of lgst_cmm_subseq of x[0..i] and y[0..j]
    // dp[0][j] = 0 = dp[i][0]
    let mut dp = vec![vec![]; lx + 1];

    // dp[i][0] = 0
    for i in 0..=lx {
        dp[i].push(0);
    }

    // dp[0][j] = 0
    dp[0] = vec![0; ly+1];

    // dp[i][j] = if x[i-1] == y[j-1] then dp[i-1][j-1] + 1 else max(dp[i-1][j], dp[i][j-1]) for 0 < i <= lx and 0 < j <= ly
    for i in 1..=lx {
        for j in 1..=ly {
            let num = if x[i-1] == y[j-1] { dp[i-1][j-1] + 1 } else { std::cmp::max(dp[i-1][j], dp[i][j-1]) };
            dp[i].push(num);
        }
    }

    dp[lx][ly]
}

fn main() {
    let data_sets = input();
    for (x,y) in data_sets {
        println!("{}", longest_common_subseq(&x, &y));
    }
}

fn input() -> Vec<(Vec<char>, Vec<char>)> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let q = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    (0..q).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let x = buf.trim().chars().collect::<Vec<char>>();
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let y = buf.trim().chars().collect::<Vec<char>>();
        (x, y)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lcs_test() {
        let (x,y) = ("".chars().collect::<Vec<_>>(), "".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 0);

        let (x,y) = ("abc".chars().collect::<Vec<_>>(), "".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 0);

        let (x,y) = ("".chars().collect::<Vec<_>>(), "abc".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 0);

        let (x,y) = ("a".chars().collect::<Vec<_>>(), "a".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 1);

        let (x,y) = ("a".chars().collect::<Vec<_>>(), "b".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 0);

        let (x,y) = ("ab".chars().collect::<Vec<_>>(), "ac".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 1);

        let (x,y) = ("ab".chars().collect::<Vec<_>>(), "ba".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 1);

        let (x,y) = ("bba".chars().collect::<Vec<_>>(), "bab".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 2);

        let (x,y) = ("abc".chars().collect::<Vec<_>>(), "abc".chars().collect::<Vec<_>>());
        assert_eq!(longest_common_subseq(&x, &y), 3);
    }
}
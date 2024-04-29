use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, m: usize,
    }
    for v in monotonically_increasing(n, m) {
        println!("{}", v.into_iter().join(" "))
    }
}

fn monotonically_increasing(n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![];
    let mut now: Vec<usize> = (1..=n).collect();
    let end: Vec<usize> = (1 + m - n..=m).collect();
    loop {
        ans.push(now.clone());
        let j = (0..n).find(|&j| now[j] == end[j]);
        match j {
            Some(j) if j == 0 => {
                break;
            }
            Some(j) => {
                now[j - 1] += 1;
                for i in j..n {
                    now[i] = now[j - 1] + (i - j) + 1;
                }
            }
            None => {
                *now.last_mut().unwrap() += 1;
            }
        }
    }
    ans
}

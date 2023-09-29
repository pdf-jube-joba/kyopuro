fn main() {
    let (k, w) = input();
    println!("{}", min(&w, k))
}

fn min(w: &[usize], k: usize) -> usize {
    let p_can = |p: usize| {
        if let Some(need) = need_capable(w, p) {
            need <= k
        } else {
            false
        }
    };
    let mut ng = 0;
    assert!(!p_can(ng));
    let mut ok = w.iter().sum();
    assert!(p_can(ok));
    while ok - ng > 1 {
        let mid = (ok - ng) / 2 + ng;
        if p_can(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn need_capable(w: &[usize], p: usize) -> Option<usize> {
    let mut count = 0;
    let mut now_w = 0;
    for wi in w {
        if *wi > p {
            return None;
        }
        if now_w + *wi <= p {
            now_w += *wi;
        } else {
            count += 1;
            now_w = *wi;
        }
    }
    Some(count + 1)
}

fn input() -> (usize, Vec<usize>) {
    let str = readline();
    let mut first_words = str.split_whitespace();
    let n = first_words.next().unwrap().parse::<usize>().unwrap();
    let k = first_words.next().unwrap().parse::<usize>().unwrap();
    let v = (0..n).map(|_|{
        readline().trim().parse::<usize>().unwrap()
    }).collect();
    (k, v)
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capable() {
        let w = vec![];
        assert_eq!(need_capable(&w, 1), Some(1));

        let w = vec![1,2,3,2,1];
        assert_eq!(need_capable(&w, 1), None);
        assert_eq!(need_capable(&w, 2), None);
        assert_eq!(need_capable(&w, 3), Some(3));
        assert_eq!(need_capable(&w, 4), Some(3));
        assert_eq!(need_capable(&w, 5), Some(3));
        assert_eq!(need_capable(&w, 6), Some(2));
        assert_eq!(need_capable(&w, 7), Some(2));
        assert_eq!(need_capable(&w, 8), Some(2));
    }
}
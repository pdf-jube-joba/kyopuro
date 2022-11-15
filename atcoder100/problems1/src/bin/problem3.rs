fn main() {
    proconio::input!{
        s: String,
    }

    println!("{}", count(&s));

}

fn count(s: &str) -> usize {
    let b: Vec<bool> = s.chars().into_iter().map(|c|{
        match c {
            'A' | 'T' | 'C' | 'G' => true,
            _ => false
        }
    }).collect();
    let n = b.len();
    (0..=n)
    .flat_map(|i|{
        (i..=n).map(move |j| (i, j))
    })
    .filter_map(|(i,j)|{
        if b[i..j].iter().all(|&b|b) {Some(j - i)} else {None}
    })
    .max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn count_test_1(){
        assert_eq!(count("ATCODER"), 3);
    }
    #[test]
    fn count_test_2(){
        assert_eq!(count("HATAGAYA"), 5);
    }
    #[test]
    fn count_test_3(){
        assert_eq!(count("SHINJUKU"), 0);
    }
}
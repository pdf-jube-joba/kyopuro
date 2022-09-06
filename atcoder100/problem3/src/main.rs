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
    let mut count = 0;
    let mut max = count;
    b.iter().for_each(|b|{
        if *b {count += 1;} else {count = 0;}
        if count > max {max = count;}
    });
    max
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
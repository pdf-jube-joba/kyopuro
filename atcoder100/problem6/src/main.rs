fn main() {
    proconio::input! {
        n: usize,
        mut s: usize,
    }

    let a: Vec<usize> = {
        let mut vec = Vec::new();
        (0..n).for_each(|_|{
            vec.push(s % 10);
            s = s / 10;
        });
        vec
    };

    println!("{}", count(&a));

}

fn count(a: &[usize]) -> usize {
    let removable = |a: &[usize], b: &[usize]| {
        let mut j = 0;
        for &i in a {
            loop {
                if j == b.len() {return false;}
                if i == b[j] {break;}
                j += 1;
            }
            j += 1;
        }
        return true;
    };
    (0..1000)
    .map(|num|{
        vec![(num  / 100) % 10, (num / 10) % 10, num % 10]
    })
    .filter(|num: &Vec<usize>|{
        removable(num, a)
    })
    .count()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn count_test_1(){
        assert_eq!(count(&[0,2,2,4]), 3);
    }
    #[test]
    fn count_test_2(){
        assert_eq!(count(&[1,2,3,1,2,3]), 17);
    }
    #[test]
    fn count_test_3(){
        assert_eq!(count(&[3,1,4,1,5,9,2,6,5,3,5,8,9,7,9,3,2,3,8]), 329);
    }
    
}
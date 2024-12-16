fn main() {
    proconio::input! {
        n: usize,
        a: [(usize, usize); n],
    }

    println!("{}", min_of(&a));

}

fn min_of(a: &[(usize, usize)]) -> usize {
    let s_min: usize = {
        let mut vec: Vec<usize> = a.iter()
            .map(|(x,_)|{*x}).collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().map(|item|{item.abs_diff(mid)}).sum()
    };
    let m_cst: usize = {
        a.iter().map(|(x, y)|{(*x).abs_diff(*y)}).sum()
    };
    let t_min: usize = {
        let mut vec: Vec<usize> = a.iter()
            .map(|(_, y)|{*y}).collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().map(|item|{item.abs_diff(mid)}).sum()
    };
    s_min + m_cst + t_min
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn min_of_test_1(){
        assert_eq!(min_of(&[(5,7), (2,6), (8,10)]), 18);
    }
    #[test]
    fn min_of_test_2(){
        assert_eq!(min_of(&[(5,7), (2,6), (8,10)]), 18);
    }
}

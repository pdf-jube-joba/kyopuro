fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    println!("{}", diff(&p, &q));

}

fn permutation<T: Clone>(first: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    struct Perm<T: Clone> {
        n: usize,
        first: Vec<T>,
        perm: Option<Vec<usize>>,
    }
    impl<T: Clone> Iterator for Perm<T> {
        type Item = Vec<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(a) = &mut self.perm {
                let n = self.n;
                let max_index = match (0..n-1).filter(|i|{a[*i] < a[i+1]}).max() {
                    Some(i) => i,
                    None => {return None;}
                };
                let swap_index = {
                    (max_index+1..n).filter(|i|{a[*i] > a[max_index]}).max().unwrap()
                };
                { // swap a[max_index] and a[swap_index]
                    let temp = a[max_index];
                    a[max_index] = a[swap_index];
                    a[swap_index] = temp;
                }
                a[max_index+1..n].reverse();
                let mut vec = Vec::new();
                a.iter().for_each(|i|{
                    vec.push(self.first[*i].clone())
                });
                Some(vec)
            } else {
                self.perm = Some((0..self.n).collect());
                Some(self.first.clone())
            }
        }
    }
    Perm {n: first.len(), first: first, perm: None }
}

fn diff(p: &[usize], q: &[usize]) -> usize {
    let mut p_num = 0;
    let mut q_num = 0;
    for (i, perm) in permutation((1..=p.len()).collect()).enumerate() {
        if *p == perm {p_num = i;}
        if *q == perm {q_num = i;}
    }
    p_num.abs_diff(q_num)
}

#[cfg(test)]
mod test {
    use crate::diff;

    #[test]
    fn diff_test_1(){
        assert_eq!(diff(&vec![1,3,2], &vec![3,1,2]), 3);
    }
    #[test]
    fn diff_test_2(){
        let a = vec![7,3,5,4,2,1,6,8];
        let b = vec![3,8,2,5,4,6,7,1];
        assert_eq!(diff(&a, &b), 17517);
    }
}
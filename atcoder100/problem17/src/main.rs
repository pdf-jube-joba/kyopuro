fn main() {
    proconio::input! {
        k: usize,
        p: [(usize, usize); k],
    }

    let result = count(&p);
    println!("{:?}", result);
    for i in 0..8 {
        let mut str = String::new();
        for j in 0..8 {
            let s = if result[i] == j {"Q"} else {"."};
            str.push_str(s);
        }
        println!("{}", str);
    }

}

fn now_stack(p: &[(usize, usize)], s: &[usize]) -> Vec<Option<usize>> {
    let mut vec = Vec::new();
    let mut index = 0;
    for i in (0..8) {
        if let Some((_,y)) = p.iter().find(|(x,y)|{*x == i}) {
            vec.push(Some(*y));
        } else if index < s.len() {
            vec.push(Some(s[index]));
            index += 1;
        } else {
            vec.push(None);
        }
    }
    vec
}

fn ok(s: &[Option<usize>], p: usize) -> bool {
    let p =
        if let Some((i,x)) = s.iter().enumerate().find_map(|(i,x)|{
            if x.is_none() {Some((i, p))} else {None}
        }) { (i,x) } else { (s.len(), p) };
    let s: Vec<(usize, usize)> = s.iter().enumerate().filter_map(|(i,x)|{
        if let Some(x) = x {Some((i,*x))} else {None}
    }).collect();
    if s.iter().map(|(x,_)|{*x}).any(|x|{x == p.0}) {return false;}
    if s.iter().map(|(_,x)|{*x}).any(|x|{x == p.1}) {return false;}
    if s.iter().map(|(x,y)|{
        (x.abs_diff(p.0), y.abs_diff(p.1))
    }).any(|(x_d, y_d)|{x_d == y_d}) {return false;}
    true
}

fn count(p: &[(usize, usize)]) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut next = 0;
    loop {
        let now = now_stack(&p, &stack);
        if now.iter().all(|i|{i.is_some()}) {
            break now.iter().map(|i|{i.unwrap()}).collect();
        }
        if ok(&now, next) {
            stack.push(next);
            next = 0;
        } else {
            if next >= 7 {
                while !(next < 7) {
                    next = stack.pop().unwrap() + 1;
                }
            } else {
                next += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn now_stack_test(){
        let a = vec![(0,0), (1,1), (5,5)];
        let b = vec![2,3];
        let c = now_stack(&a, &b);
        let d = vec![
            Some(0), Some(1), Some(2), Some(3), None, Some(5),
            None, None,
        ];
        assert_eq!(c, d)
    }
    #[test]
    fn ok_test_1(){
        let a = vec![None, None];
        assert!(ok(&a, 0));
    }
    #[test]
    fn ok_test_2(){
        let a = vec![
            Some(6),Some(0),Some(2),Some(7),Some(5),Some(3),Some(1),None
        ];
        assert!(ok(&a, 4));
    }
    #[test]
    fn count_test_1(){
        let a = vec![(0,6),(1,0),(2,2),(3,7),(4,5),(5,3),(6,1),(7,4)];
        let b = vec![6,0,2,7,5,3,1,4];
        assert_eq!(count(&a), b);
    }
    #[test]
    fn count_test_2(){
        let a = vec![(0,6),(1,0),(2,2),(3,7),(4,5),(5,3),(6,1)];
        let b = vec![6,0,2,7,5,3,1,4];
        assert_eq!(count(&a), b);
    }
    #[test]
    fn count_test_3(){
        let a = vec![(2,2),(5,3)];
        let b = [6,0,2,7,5,3,1,4];
        assert_eq!(count(&a), b);
    }
}
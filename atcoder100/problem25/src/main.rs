fn input() -> Result<Vec<Vec<bool>>, ()>{
    proconio::input! {
        w: usize, h: usize,
    }
    if w == 0 && h == 0 {return Err(());}
    let mut c = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let result: Vec<bool> = line.unwrap().split_whitespace().filter_map(|s|{
            match s {"0" => Some(false), "1" => Some(true), _ => None }}
        ).collect();
        c.push(result);
    }
    Ok(c)
}

fn main() {
    while let Ok(c) = input() {
        
    }
}

fn in_c(x: (i32, i32), c: &[Vec<bool>]) -> bool {
    let h = c.len();
    let w = c[0].len();
    if 0 <= x.0 && x.0 < h as i32 && 0 <= x.1 && x.1 < w as i32 {
        c[x.0 as usize][x.1 as usize]
    } else { return false; }
}

fn waste_c(x: (i32, i32), c: &mut [Vec<bool>]) {
    c[x.0 as usize][x.1 as usize] = false;
}

fn count(c: &mut [Vec<bool>]) -> usize {
    let h = c.len();
    let w = c[0].len();
    let mut count = 0;
    loop {
        let mut stack = Vec::new();
        let first = {
            let may_be_first =
                (0..h).map(|i|{(0..w).map(move |j|(i,j))}).flatten()
                .find(|(i,j)|{in_c((*i as i32, *j as i32), &c)});
            if let Some (first) = may_be_first { (first.0 as i32, first.1 as i32) } else {return count;}
        };
        stack.push(first);
        while let Some(next) = stack.pop() {
            waste_c(next, c);
            let ps = [(1,-1),(1,0),(1,1),(0,-1),(0,1),(-1,-1),(-1,0),(-1,1)];
            ps.iter().map(|(i,j)|{(next.0 + *i, next.1 + *j)}).for_each(|p|{
                if in_c(p, &c) {stack.push(p);}
            });
        }
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn in_c_test_1(){
        let mut c = vec![
            vec![true , false, true , false, false],
            vec![true , false, false, false, false],
            vec![true , false, true , false, true ],
            vec![true , false, false, true , false],
        ];
        assert!(in_c((0,0), &c));
        assert!(!in_c((1,1), &c));
        assert!(!in_c((-1,0), &c));
        assert!(!in_c((0,-1), &c));
        assert!(!in_c((4,0), &c));
        assert!(!in_c((0,5), &c));
    }
    #[test]
    fn waste_c_test_1(){
        let mut c = vec![
            vec![true , false, true , false, false],
            vec![true , false, false, false, false],
            vec![true , false, true , false, true ],
            vec![true , false, false, true , false],
        ];
        waste_c((0,0), &mut c);
        waste_c((0,2), &mut c);
        assert!(!c[0][0]);
    }
    #[test]
    fn count_test(){
        let mut c = vec![
            vec![true , false, true , false, false],
            vec![true , false, false, false, false],
            vec![true , false, true , false, true ],
            vec![true , false, false, true , false],
        ];
        assert_eq!(count(&mut c), 3);
    }
}
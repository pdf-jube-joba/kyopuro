use std::{collections::HashMap};

fn main() {
    proconio::input! {
        v: usize, e: usize,
        in1: [(usize, usize, usize); e],
    }
    let std = in1.into_iter().map(|(x,y,z)| ((x,y),z)).collect();
    println!("{}", match minimize(v, &std) {Some(result) => result as isize, None => -1})
}

fn minimize(v: usize, std: &HashMap<(usize, usize), usize>) -> Option<usize> {
    let mut dp: HashMap<(Vec<bool>, usize), Option<usize>> = HashMap::new();

    fn min_rec(
        std: &HashMap<(usize, usize), usize>,
        dp: &mut HashMap<(Vec<bool>, usize), Option<usize>>,
        (subset, goal): (Vec<bool>, usize)
    ) -> Option<usize> {
        assert!(subset[goal]);

        if let Some(cost) = (*dp).get(&(subset.clone(), goal)) {return *cost;}

        if subset.iter().filter(|b| **b).count() == 1 {
            let result = if goal == 0 {Some(0)} else {None};
            dp.insert((subset.clone(), goal), result);
            return result;
        }

        let next_subset = {
            let mut next_subset = subset.clone();
            next_subset[goal] = false;
            next_subset
        };

        let result = next_subset
        .iter().enumerate()
        .flat_map(|(i, &b)|{
            if b {Some((next_subset.clone(), i))} else {None}
        })
        .flat_map(|(next_subset, i)|{
            let v1 = min_rec(std, dp, (next_subset, i));
            let v2 = std.get(&(i, goal)).cloned();
            match (v1, v2) {
                (Some(cost1), Some(cost2)) => Some(cost1 + cost2),
                _ => None
            }
        })
        .min();
        (*dp).insert((subset.clone(), goal), result);
        result
    }


    let result = (1..v)
    .map(|i| (i, min_rec(std, &mut dp, (vec![true; v], i))))
    .filter_map(| (i, result)|{
        match (result, std.get(&(i,0))) {
            (Some(cost1), Some(cost2)) => Some(cost1 + cost2),
            _ => None,
        }
    })
    .min();

    result

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test_(){
        let n = 3;
        let std: HashMap<(usize, usize), usize> = vec![
            ((0,1), 1),
            ((1,2), 1),
            ((2,0), 1),
        ].into_iter().collect();
        assert_eq!(minimize(n, &std), Some(3));
        let n = 3;
        let std: HashMap<(usize, usize), usize> = vec![
            ((0,1), 1),
            ((1,2), 2),
            ((2,0), 1),
        ].into_iter().collect();
        assert_eq!(minimize(n, &std), Some(4));
    }
    #[test]
    fn minimize_test(){
        let n = 4;
        let std: HashMap<(usize, usize), usize> = vec![
            (0,1,2),
            (1,2,3),
            (1,3,9),
            (2,0,1),
            (2,3,6),
            (3,2,4),
        ].into_iter().map(|(x,y,z)| ((x,y), z)).collect();
        assert_eq!(minimize(n, &std), Some(16));
    }
}
use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        in1: [(usize, usize, u64, u64); m],
    }
    let stdt: HashMap<(usize, usize), (u64, u64)> = in1.into_iter().map(|(x,y,z,w)| ((x,y), (z,w))).collect();

    println!("{}", match minimize(n, &stdt) {
        Some((cost, num)) => {
            let mut str = String::from(cost.to_string());
            str.push_str(&num.to_string());
            str
        },
        None => "IMPOSSIBLE".to_owned(),
    });

}

fn minimize(n: usize, stdt: &HashMap<(usize, usize), (u64, u64)>) -> Option<(u64, usize)> {
    let mut dp: HashMap<(Vec<bool>, usize), Option<(u64, usize)>> = HashMap::new();

    fn min_at_first_sum(iter: impl Iterator<Item = (u64, usize)>) -> Option<(u64, usize)> {
        let mut maps: HashMap<u64, usize> = HashMap::new();
        for (cost, num) in iter {
            match maps.get_mut(&cost) {
                Some(num2) => {*num2 = num + *num2;} ,
                None => {maps.insert(cost, num);},
            }
        }
        if let Some(min_key) = maps.keys().min() {
            Some((*min_key, maps.get(min_key).cloned().unwrap()))
        } else {None}

    }

    fn min_rec(
        stdt: &HashMap<(usize, usize), (u64, u64)>,
        dp: &mut HashMap<(Vec<bool>, usize), Option<(u64, usize)>>,
        (subset, goal): (Vec<bool>, usize)
    ) -> Option<(u64, usize)> {
        assert!(subset[goal]);

        if let Some(cost) = (*dp).get(&(subset.clone(), goal)).cloned() {return cost;}

        if subset.iter().filter(|b| **b).count() == 1 {
            let result = if goal == 0 {Some((0,1))} else {None};
            dp.insert((subset.clone(), goal), result);
            return result;
        }

        let next_subset = {
            let mut next_subset = subset.clone();
            next_subset[goal] = false;
            next_subset
        };

        let iter = next_subset
        .iter().enumerate()
        .flat_map(|(i, &b)|{
            if b {Some((next_subset.clone(), i))} else {None}
        })
        .flat_map(|(next_subset, i)|{
            let v1 = min_rec(stdt, dp, (next_subset, i));
            let v2 = stdt.get(&(i, goal)).cloned();
            match (v1, v2) {
                (Some((cost1, num)), Some((cost2, time))) => {
                    if cost1 + cost2 <= time {Some((cost1 + cost2, num))} else {None}
                }
                _ => None
            }
        });

        let result = min_at_first_sum(iter);
        (*dp).insert((subset.clone(), goal), result);
        result
    }

    let iter: Vec<_> = (1..n)
    .filter_map(|i| {
        println!("i:{}", i);
        let v1 = min_rec(stdt, &mut dp, (vec![true; n], i));
        let v2 = stdt.get(&(i, 0)).cloned();
        println!("{:?} {:?}", v1, v2);
        match (v1, v2) {
            (Some((cost1, num)), Some((cost2, time))) => {
                if cost1 + cost2 <= time {Some((cost1 + cost2, num))} else {None}
            }
            _ => None,
        }
    }).collect();

    let result = min_at_first_sum(iter.into_iter());

    result

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test_1(){
        let n = 3;
        let stdt = vec![
            (1,2,1,6),
            (2,3,2,6),
            (3,1,3,6),
        ].into_iter().flat_map(|(x,y,z,w)| vec![((x-1,y-1), (z,w)), ((y-1,x-1), (z,w))] ).collect();
        assert_eq!(minimize(n, &stdt), Some((6,2)));
    }
    #[test]
    fn minimize_test_2(){
        let n = 3;
        let stdt = vec![
            (1,2,1,1),
            (2,3,1,3),
            (3,1,1,3),
        ].into_iter().flat_map(|(x,y,z,w)| vec![((x-1,y-1), (z,w)), ((y-1,x-1), (z,w))] ).collect();
        assert_eq!(minimize(n, &stdt), Some((3,1)));
    }
}
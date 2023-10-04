use std::collections::hash_set::HashSet;

fn main() {
    let a = input();
    let cost = min_cost(&a);
    println!("{}", cost);
}

fn min_cost(a: &[usize]) -> usize {
    let n = a.len();
    let total_min: usize = *a.iter().min().unwrap();

    let (s, w): (Vec<usize>, Vec<usize>) = {
        let mut a_script: Vec<(usize, usize)> = a.iter().cloned().enumerate().collect();
        a_script.sort_by(|ai, aj|{
            ai.1.cmp(&aj.1)
        });
        let mut s = Vec::with_capacity(n);
        let mut w = Vec::with_capacity(n);
        for (si, wi) in a_script {
            s.push(si);
            w.push(wi);
        }
        (s, w)
    };
    let cycles: Vec<Vec<usize>> = {
        let mut cycles = vec![];
        let mut visited: HashSet<usize> = HashSet::new();
        for i in 0..n {
            if !visited.contains(&i) {
                let mut cycle_i = vec![i];
                visited.insert(i);
                let mut next = s[i];
                while next != i {
                    cycle_i.push(next);
                    visited.insert(next);
                    next = s[next];
                }
                cycles.push(cycle_i);
            }
        }
        cycles
    };
    cycles.into_iter().map(|cycle|{
        let cycle_len = cycle.len();
        if cycle_len < 2 {
            return 0;
        }
        let cycle_min = cycle.iter().map(|&i| w[i]).min().unwrap();
        let total_cost: usize = cycle.iter().map(|&i| w[i]).sum();
        let in_cycle_case_cost = total_cost + (cycle_len - 2) * cycle_min;
        let use_total_min_cost = total_cost + cycle_min + (cycle_len + 1) * total_min;
        std::cmp::min(in_cycle_case_cost, use_total_min_cost)
    }).sum()
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.clear();
    std::io::stdin().read_line(&mut string).unwrap();
    string
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect()
}
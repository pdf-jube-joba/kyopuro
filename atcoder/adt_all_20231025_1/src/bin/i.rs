use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    }
    println!(
        "{}",
        if simulataneous_swap(a, b) {
            "Yes"
        } else {
            "No"
        }
    )
}

fn simulataneous_swap(a: Vec<usize>, b: Vec<usize>) -> bool {
    let n = a.len();
    let mut anum = vec![0; n];
    let mut bnum = vec![0; n];
    for i in 0..n {
        anum[a[i]] += 1;
        bnum[b[i]] += 1;
    }
    if anum != bnum {
        return false;
    }

    if anum.iter().any(|num| *num >= 2) {
        return true;
    }

    !(is_even_permutation(&a) ^ is_even_permutation(&b))
}

fn is_even_permutation(a: &Vec<usize>) -> bool {
    let n = a.len();
    let mut visited = vec![false; n];
    let mut cycles: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut cycle = vec![];
        let mut current = i;
        loop {
            cycle.push(current);
            visited[current] = true;
            current = a[current];
            if current == i {
                break;
            }
        }
        cycles.push(cycle)
    }
    cycles
        .into_iter()
        // .inspect(|cycle| eprintln!("{:?} {}", &cycle, (cycle.len() - 1) % 2 == 0))
        .map(|cycle| (cycle.len() - 1) % 2 == 0)
        .fold(true, |acc, x| !(acc ^ x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evp_test() {
        assert!(is_even_permutation(&vec![0, 1]));
        assert!(!is_even_permutation(&vec![1, 0]));
        assert!(is_even_permutation(&vec![0, 1, 2]));
        assert!(!is_even_permutation(&vec![2, 1, 0]));
        assert!(is_even_permutation(&vec![2, 0, 1]));
    }
}

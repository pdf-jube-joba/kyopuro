fn main() {
    proconio::input! {
        n: usize, q: usize,
        mut r: [usize; n],
        queries: [usize; q],
    }
    let res = num_of_sleigh(r, queries);
    for res in res {
        println!("{}", res);
    }
}

fn num_of_sleigh(mut r: Vec<usize>, queries: Vec<usize>) -> Vec<usize> {
    r.sort();
    // sum_r[i \in 0..=r.len()] = if i == 0 then 0 else r[0] + ... + r[i-1]
    let sum_r = {
        let mut v = vec![0];
        let mut sum = 0;
        for i in 0..r.len() {
            sum += r[i];
            v.push(sum);
        }
        v
    };

    // find max m of sum_r[m] = r_0 + .. + r_{m-1} <= x
    queries 
        .into_iter()
        .map(|x| match sum_r.binary_search(&x) {
            // case: sum_r[m] == x
            Ok(i) => i,
            // case: sum_r[m-1] < x < sum_r[m]
            // m > 0 because sum_r[0] = 0 <= x
            Err(e) => e - 1,
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(num_of_sleigh(vec![2,3], vec![2,3,5]), vec![1,1,2]);
        assert_eq!(num_of_sleigh(vec![2], vec![1,2,3]), vec![0,1,1]);
    }
}
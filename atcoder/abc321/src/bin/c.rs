use itertools::Itertools;

fn main() {
    proconio::input! {
        k: usize,
    }
    let all = all_321_num();
    println!("{}", all[k-1]);
}

fn all_321_num() -> Vec<usize> {
    let mut v = vec![];
    for i in 0..(1 << 10) {
        let mut nums = (0_usize..10).filter(|j| (i & (1 << j)) != 0).collect_vec();
        nums.sort();
        let num: usize = nums
            .into_iter()
            .enumerate()
            .map(|(j, num)| 10_usize.pow(j as u32) * num)
            .sum();
        if num != 0 {
            v.push(num);
        }
    }
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use crate::all_321_num;

    #[test]
    fn a() {
        let all = all_321_num();
        println!("{:?}", all);
    }
}

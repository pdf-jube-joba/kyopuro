fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    for ans in k1th_largest_numer(a) {
        println!("{ans}")
    }
}

fn k1th_largest_numer(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    // num_ords[i] = (l[i], #{i | A[i] = l[i]})
    // where l[i] = a[i] ordered by largest exluded deplicate
    let nums_ords: Vec<(usize, usize)> = {
        let mut v: Vec<usize> = a.clone();
        v.sort();
        let mut n = vec![];
        while let Some(p) = v.pop() {
            let mut count = 1;
            while let Some(&d) = v.last() {
                if d != p {
                    break;
                }
                v.pop();
                count += 1
            }
            n.push((p, count))
        }
        n
    };
    let l = nums_ords.len();
    nums_ords
        .into_iter()
        .map(|n| n.1)
        .chain(vec![0; n - l])
        .collect()
}

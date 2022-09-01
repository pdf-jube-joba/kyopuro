fn main() {
    proconio::input!{
        n: usize, m: usize,
        a: [[usize; m]; n]
    }

    let mut max = 0;
    for i in 0..m {
        for j in i..m {
            let mut count = 0;
            for k in 0..n {
                count += std::cmp::max(a[k][i], a[k][j])
            }
            if count > max {max = count;}
        }
    }

    println!("{}", max)

}

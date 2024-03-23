fn main() {
    proconio::input! {
        n: usize, x: usize, y: usize,
        a: [usize; n],
    }
    println!("{}", max_min((x, y), a))
}

fn max_min((x, y): (usize, usize), a: Vec<usize>) -> usize {
    let mut count = 0;
    for range in a.split(|&ai| ai < y || x < ai) {
        let n = range.len();
        let mut l = 0;
        let mut r = 0;
        let mut countx = 0;
        let mut county = 0;
        while l < n {
            while r < n && (countx == 0 || county == 0) {
                if range[r] == x {
                    countx += 1;
                }
                if range[r] == y {
                    county += 1;
                }
                r += 1;
            }
            if countx > 0 && county > 0 {
                count += n + 1 - r;
            }
            if range[l] == x {
                countx -= 1;
            }
            if range[l] == y {
                county -= 1;
            }
            l += 1;
        }
    }
    count
}

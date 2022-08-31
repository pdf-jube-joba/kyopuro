use proconio;

fn main() {

    proconio::input! {
        n: usize, l: usize,
        k: usize,
        a: [usize; n],
    }

    let vec = {
        let mut vec = Vec::with_capacity(n);
        vec.push(a[0]);
        for i in 1..n {
            vec.push(a[i] - a[i-1]);
        }
        vec.push(l-a[n-1]);
        vec
    };

    let mut able_min = 0;
    let mut unable_max = l+1;
    while unable_max - able_min > 1 {
        let middle = (unable_max + able_min) / 2;
        let num = number(&vec, middle);
        if num < k+1 {
            unable_max = middle;
        } else {
            able_min = middle;
        }
    }

    println!("{}", able_min);
}

fn number(a: &[usize], lower: usize) -> usize {
    number_rec(a, 0, lower)
}

fn number_rec(a: &[usize], begin: usize, lower: usize) -> usize {
    let n = a.len();
    let mut sum = 0;
    let mut now = begin;
    while !(lower <= sum) {
        if now >= n {
            return 0;
        }
        sum += a[now];
        now += 1;
    }
    1 + number_rec(a, now, lower)
}
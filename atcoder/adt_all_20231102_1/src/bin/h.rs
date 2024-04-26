fn main() {
    proconio::input! {
        n: usize, m: usize, e: usize,
        uv: [(usize, usize); e],
        q: usize,
        x: [usize; q],
    }
    for ans in blackout2((n, m), uv, x) {
        println!("{ans}");
    }
}

fn blackout2((n, m): (usize, usize), uv: Vec<(usize, usize)>, x: Vec<usize>) -> Vec<usize> {
    todo!()
}

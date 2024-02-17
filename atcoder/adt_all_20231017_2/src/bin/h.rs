use num_integer::{lcm, Integer};

fn main() {
    proconio::input! {
        n: usize, x: usize, y: usize,
        pt: [(usize, usize); n-1],
        q: usize,
        qs: [usize; q],
    }
    let pre = Pre::new(pt);
    for q in qs {
        println!("{}", q + x + pre.duration(q + x) + y)
    }
}

struct Pre {
    q: Vec<usize>,
}

impl Pre {
    fn new(pt: Vec<(usize, usize)>) -> Self {
        let lcm = pt.iter().map(|(p, t)| *p).reduce(lcm).unwrap();
        let q = (0..lcm)
            .map(|start| {
                let mut time = start;
                for &(p, t) in &pt {
                    time = num_integer::div_ceil(time, p) * p + t;
                }
                time - start
            })
            .collect();
        Self { q }
    }
    fn duration(&self, q: usize) -> usize {
        let l = self.q.len();
        self.q[q % l]
    }
}

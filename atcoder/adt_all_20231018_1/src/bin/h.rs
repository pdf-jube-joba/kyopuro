fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }
}

// angle of (x in ZZ_{>= 0}, y in ZZ_{(>= 0)})
//
fn angle((x, y): (usize, usize)) -> f64 {
    if x == 0 {
        std::f64::consts::FRAC_2_PI
    } else {
        (y as f64 / x as f64).atan()
    }
}

fn max_of_take(mut xy: Vec<(usize, usize)>) -> usize {
    let n = xy.len();
    let mut xy_angles: Vec<(f64, f64)> = xy
        .iter()
        .map(|&(x, y)| (angle((x, y - 1)), angle((x - 1, y))))
        .collect();
    xy_angles.sort_by(|&(l1, r1), &(l2, r2)|{
        l1.partial_cmp(&l2).unwrap().then(r1.partial_cmp(&r2).unwrap())
    });
    let mut r_max: f64 = 0_f64;
    let mut count: usize = 0;
    loop {
        // pos satisfies angle of r_max <= (x[i], y[i]-1) and min of such i
        let pos = xy.partition_point(|&(x, y)| angle((x - 1, y)) <= r_max);

        if pos == n {
            break;
        }
        count += 1;
        r_max = {
            let (x, y) = xy[pos];
            angle((x - 1, y))
        };
    }
    count
}

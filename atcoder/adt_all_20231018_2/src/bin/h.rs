fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    println!("{}", max_of_take(xy))
}

// angle of (x in ZZ_{>= 0}, y in ZZ_{(>= 0)})
//
fn angle((x, y): (usize, usize)) -> f64 {
    if x == 0 {
        std::f64::consts::FRAC_PI_2
    } else {
        (y as f64 / x as f64).atan()
    }
}

fn max_of_take(mut xy: Vec<(usize, usize)>) -> usize {
    let n = xy.len();

    // xy_angles[i] = (l[i] = angle of (x, y-1) in [0.. pi/2], r[i] = angle of (x-1, y) in [0.. pi/2])       
    let mut xy_angles: Vec<(f64, f64)> = xy
        .iter()
        .map(|&(x, y)| (angle((x, y - 1)), angle((x - 1, y))))
        .collect();
    
    // open interval scheduling with (l[i], r[i])
    
    // sort by angle of (x-1, y)
    xy_angles.sort_by(|&(l1, r1), &(l2, r2)| {
        r1.partial_cmp(&r2)
            .unwrap()
            .then(l1.partial_cmp(&l2).unwrap())
    }); 

    eprintln!("{:?}", xy_angles);

    let mut r_max: f64 = 0_f64;
    let mut ind: usize = 0;
    let mut count: usize = 0;

    'main: loop {
        // search i s.t r_max < l[i] and r[i] is min of such i
        'search: loop {
            // search complete
            if ind == n {
                break 'main;
            }
            if r_max <= xy_angles[ind].0 {
                break 'search;
            }
            ind += 1;
        }
        eprintln!("{r_max} {ind}");
        // choose ind
        count += 1;
        r_max = xy_angles[ind].1;
    }

    count
}

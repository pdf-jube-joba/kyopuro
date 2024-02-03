use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        sc: [(usize, usize); n],
    }
    println!("{}", min_of_num(sc));
}

fn min_of_num(sc: Vec<(usize, usize)>) -> usize {
    // map[l: odd] = number of slimes when converted to the smallest size of that slime
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (mut s, c) in sc {
        let mut t = 0;
        while s % 2 == 0 {
            t += 1;
            s /= 2;
        }
        // (old s) == (new s) * (2^t) where new s: odd
        let ref_to_val = map.entry(s).or_insert(0);
        *ref_to_val += 2_usize.pow(t) * c;
    }
    map.into_iter()
        .map(|(odd, num_slime)| num_slime.count_ones() as usize)
        .sum()
}

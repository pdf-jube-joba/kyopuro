use im_rc::HashMap;
use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, Usize1); q],
    }
    let ans = query(a, xk);
    for ans in ans {
        println!(
            "{}",
            if let Some(k) = ans {
                (k + 1).to_string()
            } else {
                "-1".to_owned()
            }
        );
    }
}

fn query(a: Vec<usize>, xk: Vec<(usize, usize)>) -> Vec<Option<usize>> {
    let mut map_vec: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &ai) in a.iter().enumerate() {
        let t = map_vec.entry(ai).or_default();
        t.push(i);
    }
    xk.into_iter()
        .map(|(x, k)| {
            let v = map_vec.get(&x)?;
            v.get(k).cloned()
        })
        .collect()
}

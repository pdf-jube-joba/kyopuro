fn main() {
    let (x, y) = input();
    println!("{}", diff_p(&x, &y, Some(1)));
    println!("{}", diff_p(&x, &y, Some(2)));
    println!("{}", diff_p(&x, &y, Some(3)));
    println!("{}", diff_p(&x, &y, None));
}

struct NdimPoint {
    p: Vec<f64>,
}

fn diff_p(x: &NdimPoint, y: &NdimPoint, p: Option<usize>) -> f64 {
    if x.p.len() != y.p.len() {
        panic!();
    }
    let iter = x.p.iter().zip(y.p.iter()).map(|(xi, yi)|{
        (xi - yi).abs()
    });
    match p {
        Some(p) => iter.map(|di| di.powi(p as i32)).sum::<f64>().powf(1_f64 / (p as f64)),
        None => {
            let mut max = std::f64::MIN;
            for di in iter {
                if !di.is_nan() && max <= di {
                    max = di;
                }
            }
            max
        }
    }
}

fn input() -> (NdimPoint, NdimPoint) {
    let _n = readline().trim().parse::<usize>().unwrap();
    let x = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap() as f64).collect();
    let y = readline().split_whitespace().map(|str| str.parse::<usize>().unwrap() as f64).collect();
    (
        NdimPoint { p: x },
        NdimPoint { p: y },
    )
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
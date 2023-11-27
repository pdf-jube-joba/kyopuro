fn main() {
    let (mut w, vw) = input();

    // vec of (vi, wi) whose order is value per weight
    // let (vk, wk) = nice_order[k]
    // then i <= j <=> val/weight of i <= val/weight of j
    let mut nice_order: Vec<(usize, usize)> = {
        let mut v: Vec<(usize, usize)> = vw;
        v.sort_by(|(v1, w1), (v2, w2)| (v1 * w2).cmp(&(v2 * w1))); // (v1/w1) cmp (v2/w2) <=> v1*w2 cmp v2*w1
        v
    };

    let mut val: f64 = 0.0;

    while w > 0 {
        // last element of nice_order is most valuable per weight 
        if  let Some((vi, wi)) = nice_order.pop() {
            // pack until we can
            if w >= wi {
                w -= wi;
                val += vi as f64;
            } else {
                val += w as f64 * (vi as f64 / wi as f64);
                break;
            }
        
        // if not exists, it end
        } else {
            break;
        }
    }

    println!("{}", val);
}

fn input() -> (usize, Vec<(usize, usize)>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    let (n, w) = {
        stdin.read_line(&mut buf).unwrap();
        let v: Vec<_> = buf
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect();
        (v[0], v[1])
    };

    (
        w,
        (0..n)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();
                let v: Vec<_> = buf
                    .split_whitespace()
                    .map(|str| str.parse::<usize>().unwrap())
                    .collect();
                (v[0], v[1])
            })
            .collect(),
    )
}

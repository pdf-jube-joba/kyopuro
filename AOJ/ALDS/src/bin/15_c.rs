fn main() {
    let mut st: Vec<(usize, usize)> = input();
    st.sort_by_key(|(_si, ti)| *ti);

    let mut count = 0;
    let mut prev_enjoy_num: Option<usize> = None;

    loop {
        // where to start search index and when prev activity is end
        let (next_start, from_ti) = {
            match prev_enjoy_num {
                Some(i) => (i + 1, Some(st[i].1 + 1)),
                None => (0, None),
            }
        };
        // next number of we sould participate (the earliest time of end in we can participate)
        let next_enjoy = &st[next_start..]
            .iter()
            .enumerate()
            .find_map(|(i, (si, ti))| {
                if from_ti <= Some(*si) {
                    Some(next_start + i)
                } else {
                    None
                }
            });

        if let Some(num) = next_enjoy {
            count += 1;
            prev_enjoy_num = Some(*num);
        } else {
            break;
        }
    }

    println!("{}", count);
}

fn input() -> Vec<(usize, usize)> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

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
        .collect()
}

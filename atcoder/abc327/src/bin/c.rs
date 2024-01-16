fn main() {
    proconio::input! {
        a: [[usize; 9]; 9]
    }
    let b = check_number_place(&a);
    println!("{}", if b {"Yes"} else {"No"})
}

fn check_number_place(a: &[Vec<usize>]) -> bool {
    let is_full = |mut v: Vec<usize>| -> bool {
        v.sort();
        v.dedup();
        v.len() == 9
    };
    // check row
    for i in 0..9 {
        let mut v = vec![];
        for j in 0..9 {
            v.push(a[i][j]);
        }
        if !is_full(v) {
            return false;
        }
    }
    // check col
    for i in 0..9 {
        let mut v = vec![];
        for j in 0..9 {
            v.push(a[j][i]);
        }
        if !is_full(v) {
            return false;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut v = vec![];
            for i1 in 0..3 {
                for j1 in 0..3 {
                    v.push(a[3*i+i1][3*j+j1]);
                }
            }
            if !is_full(v) {
                return false;
            }
        }
    }
    true
}

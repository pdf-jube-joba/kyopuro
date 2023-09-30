fn main() {
    let mut s = input();
    let right = s.len();
    let cmp_num = mergesort(&mut s, 0, right);
    println!("{}", cmp_num);
}

// merge a[l, m) and a[m, r)
fn merge(a: &mut[usize], left: usize, mid: usize, right: usize) -> usize {
    let mut l: Vec<usize> = a[left..mid].to_owned();
    l.push(std::usize::MAX);
    let mut r: Vec<usize> = a[mid..right].to_owned();
    r.push(std::usize::MAX);
    let mut i = 0;
    let mut j = 0;
    let mut inv = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
            if std::usize::MAX != l[i] {
                inv += j;
            }
        } else {
            a[k] = r[j];
            if std::usize::MAX != l[i] {
                inv += 1;
            }
            j += 1;
        }
    }
    inv
}

// sort a[l, r) 
fn mergesort(a: &mut[usize], left: usize, right: usize) -> usize {
    if left + 1 < right {
        let mid = (left + right) / 2;
        let l_num = mergesort(a, left, mid);
        let r_num = mergesort(a, mid, right);
        let m_num = merge(a, left, mid, right);
        l_num + r_num + m_num
    } else {
        0
    }
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.clear();
    std::io::stdin().read_line(&mut string).unwrap();
    string.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
}
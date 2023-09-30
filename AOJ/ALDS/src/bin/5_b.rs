fn main() {
    let mut s = input();
    let right = s.len();
    let cmp_num = mergesort(&mut s, 0, right);
    for (i, si) in s.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", si);
    }
    println!();
    println!("{}", cmp_num);
}

fn merge(a: &mut[usize], left: usize, mid: usize, right: usize) -> usize {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l: Vec<usize> = (0..n1).map(|i| a[left + i]).collect();
    let mut r: Vec<usize> = (0..n2).map(|i| a[mid + i]).collect();
    l.push(std::usize::MAX);
    r.push(std::usize::MAX);
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }
    right - left
}

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
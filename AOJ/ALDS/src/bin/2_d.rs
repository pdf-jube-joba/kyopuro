fn main() {
    let mut a = input();
    let n = a.len();
    let g = power_two(n);
    let cnt = shell_sort(&mut a, &g);
    println!("{}", g.len());
    print_vec(&g);
    println!("{}", cnt);
    for i in a {
        println!("{}", i);
    }
}

fn print_vec(a: &[usize]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
        if i != a.len() - 1 {
            print!(" ");
        }
    }
    println!()
}

fn power_two(n: usize) -> Vec<usize> {
    let seq_i = |i: usize| 2_usize.pow(i as u32);
    let mut i: usize = 0;
    while seq_i(i) <= n {
        i += 1;
    }
    (0..i).rev().map(seq_i).collect()
}

fn power_e(n: usize) -> Vec<usize> {
    let seq_i = |i: usize| {
        std::f64::consts::E.powf(i as f64) as usize
    };
    let mut i: usize = 0;
    while seq_i(i) <= n {
        i += 1;
    }
    (0..i).rev().map(seq_i).collect()
}

fn shell_sort(a: &mut[usize], g: &[usize]) -> usize {
    let mut cnt = 0;
    for gi in g {
        cnt += insersion(a, *gi);
    }
    cnt
}

fn insersion(a: &mut[usize], g: usize) -> usize {
    let n = a.len();
    let mut cnt = 0;
    for i in g..n {
        let v = a[i];
        let mut j: isize = i as isize - g as isize;
        while j >= 0 && a[j as usize] > v {
            a[(j + (g as isize)) as usize] = a[j as usize];
            j -= g as isize;
            cnt += 1;
        }
        a[(j + (g as isize)) as usize] = v;
    }
    cnt
}

fn input() -> Vec<usize> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut string).unwrap();
    let n = string.trim().parse::<usize>().unwrap();
    (0..n).map(|_|{
        string = String::new();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<usize>().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power(){
        eprintln!("{:?}", power_e(3));
    }
}
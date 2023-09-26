use rand::Rng;

fn main() {
    let a = gen();
    let n = a.len();

    let power_2 = cut_and_rev(n, |i: usize| 2_usize.pow(i as u32));
    let power_3 = cut_and_rev(n, |i: usize| 3_usize.pow(i as u32));
    let power_e = cut_and_rev(n, |i: usize| {
        std::f64::consts::E.powf(i as f64) as usize
    });
    let e_frac = cut_and_rev(n, |i: usize| {
        if i == 0 {
            1
        } else {
            std::f64::consts::E.powf((i + 1) as f64) as usize / 2
        }
    });
    let knuth = cut_and_rev(n, |i: usize| (3_usize.pow(i as u32) - 1) / 2);

    let mut b = a.clone();

    print!("power 2");
    measure(&mut b, &power_2);

    b = a.clone();
    print!("power 3");
    measure(&mut b, &power_3);
    
    b = a.clone();
    print!("power e");
    measure(&mut b, &power_e);
    
    b = a.clone();
    print!("e^k/2");
    measure(&mut b, &e_frac);
    
    b = a.clone();
    print!("(3^k-1)/2");
    measure(&mut b, &knuth);
    
}

fn measure(a: &mut[usize], g: &[usize]) {
    println!("start");
    let start = std::time::Instant::now();
    let cnt = shell_sort(a, g);
    let end = std::time::Instant::now();
    println!("end cnt:{} time:{:?}", cnt, end.duration_since(start));
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

fn cut_and_rev<T: Fn(usize) -> usize>(n: usize, f: T) -> Vec<usize> {
    let mut i: usize = 0;
    while f(i) <= n {
        i += 1;
    }
    (0..i).rev().map(f).collect()
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

fn gen() -> Vec<usize> {
    let n = 1_000_000;
    let mut vec = vec![0; n];
    rand::thread_rng().try_fill(&mut vec[..]).unwrap();
    vec
}
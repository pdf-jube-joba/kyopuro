enum Grade {
    A, B, C, D, F,
}

impl ToString for Grade {
    fn to_string(&self) -> String {
        match self {
            Grade::A => "A".to_string(),
            Grade::B => "B".to_string(),
            Grade::C => "C".to_string(),
            Grade::D => "D".to_string(),
            Grade::F => "F".to_string(),
        }
    }
}

fn main() {
    loop {
        let (m, f, r) = input();
        if m.is_none() && f.is_none() && r.is_none() {
            break;
        }
        let grade = grade(m, f, r);
        println!("{}", grade.to_string());
    }
}

fn grade(m: Option<usize>, f: Option<usize>, r: Option<usize>) -> Grade {
    let m = match m {
        Some(m) => m,
        None => return Grade::F,
    };
    let f = match f {
        Some(f) => f,
        None => return Grade::F,
    };
    if m + f >= 80 {
        Grade::A
    } else if m + f >= 65 {
        Grade::B
    } else if m + f >= 50 {
        Grade::C
    } else if m + f >= 30 {
        match r {
            Some(r) if r >= 50 => Grade::C,
            _ => Grade::D
        }
    } else {
        Grade::F
    }
}

fn input() -> (Option<usize>, Option<usize>, Option<usize>) {
    let vec: Vec<Option<usize>> = read_line().split_whitespace().map(|str|{str.parse::<usize>().ok()}).collect();
    (vec[0], vec[1], vec[2])
}

fn read_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}
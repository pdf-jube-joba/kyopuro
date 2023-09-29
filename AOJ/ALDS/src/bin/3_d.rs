enum Top {
    U, F, D,
}

use std::str::FromStr;
impl FromStr for Top {
    type Err = ();
    fn from_str(str: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        match str {
            "/" => Ok(Top::U),
            "\\" => Ok(Top::D),
            "_" => Ok(Top::F),
            _ => Err(()),
        }
    }
}

fn get_corr_u(ts: &[Top]) -> Option<usize> {
    let mut corr = 1;
    for (i, t) in ts.iter().enumerate() {
        match t {
            Top::U => {
                corr -= 1;
            }
            Top::F => {}
            Top::D => {
                corr += 1;
            }
        }
        if corr == 0 {
            return Some(i);
        }
    }
    None
}

struct PondStack(Vec<usize>);

impl PondStack {
    fn new() -> PondStack {
        PondStack(vec![])
    }
    fn stack_ext(&mut self) {
        self.0.push(0);
    }
    fn stack_plus(&mut self, plus: usize) {
        let len = self.0.len();
        self.0[len - 1] += plus;
    }
}

use std::convert::From;
impl From<PondStack> for Vec<usize> {
    fn from(value: PondStack) -> Vec<usize> {
        value.0
    }
}

fn count(ts: Vec<Top>) -> Vec<usize> {
    let corr_dus: Vec<(usize, usize)> = ts.iter().enumerate().filter_map(|(i, t)|{
        if let Top::D = t {
            if let Some(i2) = get_corr_u(&ts[i+1..]) {
                Some((i, i + 1 + i2))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
    let mut ponds = PondStack::new();
    let mut till = 0;
    for (d, u) in corr_dus {
        if u < till {
            ponds.stack_plus(u - d);
        } else {
            ponds.stack_ext();
            ponds.stack_plus(u - d);
            till = u;
        }
    }
    ponds.into()
}

fn main() {
    let ts = input();
    let count = count(ts);
    let a = count.iter().sum::<usize>();
    let k = count.len();
    println!("{}", a);
    print!("{}", k);
    for p in count {
        print!(" {}", p);
    }
    println!()
}

fn input() -> Vec<Top> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.chars().filter_map(|char: char| char.to_string().parse::<Top>().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stack() {
        let stack: Vec<Top> = "\\/".chars().map(|char: char| char.to_string().parse::<Top>().unwrap()).collect();
        let count_num = count(stack);
        assert_eq!(count_num, vec![1]);

        let stack: Vec<Top> = "\\\\/\\//".chars().map(|char: char| char.to_string().parse::<Top>().unwrap()).collect();
        let count_num = count(stack);
        assert_eq!(count_num, vec![7]);

        let stack: Vec<Top> = "\\/\\/\\//".chars().map(|char: char| char.to_string().parse::<Top>().unwrap()).collect();
        let count_num = count(stack);
        assert_eq!(count_num, vec![1,1,1]);
    }
}
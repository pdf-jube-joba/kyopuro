#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Minus,
    Multi,
    Div,
    End,
}

impl Op {
    fn op(self, a: isize, b: isize) -> Option<isize> {
        match self {
            Op::Add => Some(a + b),
            Op::Minus => Some(a - b),
            Op::Multi => Some(a * b),
            Op::Div => Some(a / b),
            Op::End => None,
        }
    }
}

use std::convert::TryFrom;

impl TryFrom<&str> for Op {
    type Error = String;
    fn try_from(value: &str) -> Result<Op, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Minus),
            "*" => Ok(Op::Multi),
            "/" => Ok(Op::Div),
            "?" => Ok(Op::End),
            _ => Err("failed".to_string()),
        }
    }
}

fn main() {
    loop {
        let (a, op, b) = input();
        if op == Op::End {
            break;
        }
        let res = op.op(a as isize, b as isize);
        println!("{}", res.unwrap());
    }
}

fn input() -> (usize, Op, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<&str> = string.split_whitespace().collect();
    let a = vec[0].parse().unwrap();
    let op = Op::try_from(vec[1]).unwrap();
    let b = vec[2].parse().unwrap();
    (a, op, b)
}
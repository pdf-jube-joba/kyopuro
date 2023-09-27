enum Operator {
    Plus,
    Minus,
    Multi,
}

use std::convert::TryFrom;

impl TryFrom<&str> for Operator {
    type Error = ();
    fn try_from(value: &str) -> Result<Operator, ()> {
        match value {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Multi),
            _ => Err(()),
        }
    }
}

enum Either {
    Operator(Operator),
    Operand(usize),
}

fn main() {
    let input = input();
    let mut stack: Vec<isize> = vec![];
    for op in input {
        match op {
            Either::Operand(u) => {
                stack.push(u as isize);
            }
            Either::Operator(op) => {
                match op {
                    Operator::Plus => {
                        let v0 = stack.pop().unwrap();
                        let v1 = stack.pop().unwrap();
                        let res = v1 + v0;
                        stack.push(res);
                    }
                    Operator::Minus => {
                        let v0 = stack.pop().unwrap();
                        let v1 = stack.pop().unwrap();
                        let res = v1 - v0;
                        stack.push(res);
                    }
                    Operator::Multi => {
                        let v0 = stack.pop().unwrap();
                        let v1 = stack.pop().unwrap();
                        let res = v1 * v0;
                        stack.push(res);
                    }
                }
            }
        }
    }
    println!("{}", stack.pop().unwrap());
}

fn input() -> Vec<Either> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.split_whitespace().map(|str|{
        match str.parse::<usize>() {
            Ok(u) => Either::Operand(u),
            Err(_) => {
                let operator = Operator::try_from(str).unwrap();
                Either::Operator(operator)
            }
        }
    }).collect()
}

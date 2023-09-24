fn main() {
    let (mut str, queries) = input();
    for query in queries {
        match query {
            Query::Print(a, b) => {
                println!("{}", &str[a..=b]);
            }
            Query::Reverse(a, b) => {
                let rev_a_b: String = (&str[a..=b]).chars().rev().collect();
                str.replace_range(a..=b, &rev_a_b);
            }
            Query::Replace(a, b, p) => {
                str.replace_range(a..=b, &p);
            }
        }
    }
}

enum Query {
    Print(usize, usize),
    Reverse(usize, usize),
    Replace(usize, usize, String),
}

use std::convert::TryFrom;
impl TryFrom<&str> for Query {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn parse_usize(value: &str) -> usize {
            value.trim().parse::<usize>().unwrap()
        }
        let words: Vec<&str> = value.split_whitespace().collect();
        match words[0] {
            "print" => {
                Ok(Query::Print(parse_usize(words[1]), parse_usize(words[2])))
            }
            "reverse" => {
                Ok(Query::Reverse(parse_usize(words[1]), parse_usize(words[2])))
            }
            "replace" => {
                Ok(Query::Replace(parse_usize(words[1]), parse_usize(words[2]), words[3].to_owned()))
            }
            _ => {
                Err(value.to_owned())
            }
        }
    }
}

fn input() -> (String, Vec<Query>) {
    let mut string = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut string).unwrap();
    let target: String = string.clone();

    let q: usize = {
        string = String::new();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<usize>().unwrap()
    };

    let queries: Vec<Query> = (0..q).map(|_|{
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        Query::try_from(string.trim()).unwrap()
    }).collect();

    (target, queries)
}
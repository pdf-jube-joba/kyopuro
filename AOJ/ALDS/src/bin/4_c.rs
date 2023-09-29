enum Order<T> {
    Insert(T),
    Find(T),
}

#[derive(Debug, Clone, Copy)]
enum ACGT {
    A = 0, C = 1, G = 2, T = 3,
}

struct ACGTString {
    v: Vec<ACGT>,
}

impl ACGTString {
    fn new(v: Vec<ACGT>) -> Option<Self> {
        if v.len() <= 12 {
            Some( Self { v } )
        } else {
            None
        }
    }
}

trait HasHashWithFixedLen {
    fn all_len() -> usize;
    fn hash(&self) -> usize;
}

impl HasHashWithFixedLen for ACGTString {
    fn all_len() -> usize {
        4_usize.pow(13)
    }
    fn hash(&self) -> usize {
        self.v.iter().enumerate().map(|(i, acgt)|{
            4_usize.pow(i as u32) * (*acgt as usize + 1)
        }).sum::<usize>() - 1
    }
}

use std::marker::PhantomData;
struct HashSetFixedLen<T> where T: HasHashWithFixedLen {
    v: Vec<bool>,
    d: PhantomData<T>,
}

impl<T> HashSetFixedLen<T> where T: HasHashWithFixedLen {
    fn new() -> Self {
        let v_len = <T as HasHashWithFixedLen>::all_len();
        Self { 
            v: vec![false; v_len],
            d: PhantomData
        }
    }
    fn find(&self, elem: &T) -> bool {
        self.v[elem.hash()]
    }
    fn insert(&mut self, elem: T) {
        self.v[elem.hash()] = true;
    }
    fn delete(&mut self, elem: T) {
        self.v[elem.hash()] = true;
    }
}

fn main() {
    let orders = input();
    let mut new_hash = HashSetFixedLen::<ACGTString>::new();
    for order in orders {
        match order {
            Order::Insert(acgt_str) => {
                new_hash.insert(acgt_str);
            }
            Order::Find(acgt_str) => {
                if new_hash.find(&acgt_str) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
        }
    }
}

fn into_vec_acgt(str: &str) -> Vec<Option<ACGT>> {
    str.chars().map(|char: char|{
        match char {
            'A' => Some(ACGT::A),
            'C' => Some(ACGT::C),
            'G' => Some(ACGT::G),
            'T' => Some(ACGT::T),
            _ => None,
        }
    }).collect()
}

fn input() -> Vec<Order<ACGTString>> {
    let n = readline().trim().parse::<usize>().unwrap();
    (0..n).map(|_|{
        let str = readline();
        let mut spl = str.split_whitespace();
        match spl.next().unwrap() {
            "insert" => Order::Insert(
                ACGTString::new(
                    into_vec_acgt(spl.next().unwrap()).into_iter().filter_map(|b| b).collect()
                ).unwrap()
            ),
            "find" => Order::Find(
                ACGTString::new(
                    into_vec_acgt(spl.next().unwrap()).into_iter().filter_map(|b| b).collect()
                ).unwrap()
            ),
            _ => panic!(),
        }
    }).collect()
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let acgt_str: ACGTString = ACGTString::new(
            vec![ACGT::A]
        ).unwrap();
        assert_eq!(acgt_str.hash(), 0);

        let acgt_str: ACGTString = ACGTString::new(
            vec![ACGT::C, ACGT::G]
        ).unwrap();
        assert_eq!(acgt_str.hash(), 13);

        let acgt_str: ACGTString = ACGTString::new(
            vec![ACGT::C, ACGT::A, ACGT::T]
        ).unwrap();
        assert_eq!(acgt_str.hash(), 69);
    }
}
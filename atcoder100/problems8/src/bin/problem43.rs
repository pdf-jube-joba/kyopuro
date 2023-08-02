use std::{collections::HashMap, ops::Index};

fn main() {
    proconio::input! {
        n: usize,
        in1: [[char; n]; 5],
    }
    let s: Vec<Vec<Option<Color>>> =
        (0..n).map(|i|
            (0..5).map(|j| match in1[j][i] {
                'R' => Some(Color::Red),
                'B' => Some(Color::Blue),
                'W' => Some(Color::White),
                '#' => None,
                _ => unreachable!()
            }).collect()
        ).collect();
    
    println!("{}", minimize(&s));
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Color {
    Red, Blue, White
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ColorIndexed<T> {
    red: T, blue: T, white: T
}

impl<T> Index<Color> for ColorIndexed<T> {
    type Output = T;
    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::Red => &self.red,
            Color::Blue => &self.blue,
            Color::White => &self.white,
        }
    }
}

// s[i] で "i 列目" をあらわす
fn minimize(s: &[Vec<Option<Color>>]) -> usize {
    let all_color = vec![Color::Red, Color::Blue, Color::White];

    let n = s.len();
    let mut dp: Vec<ColorIndexed<usize>> = Vec::with_capacity(n);
    for i in 0..n {
        dp.push({
            if i > 0 {
                let c_next = |c:Color| -> usize {
                    let c_num = s[i]
                        .iter().filter(|c2| c2.as_ref().map_or(false, |c2| c == *c2)).count();
                    all_color
                    .iter().filter(|c2| c != **c2)
                    .map(|c2|{
                        dp[i-1][c2.clone()] + 5 - c_num
                    })
                    .min().unwrap()
                };
                ColorIndexed{
                    red: c_next(Color::Red),
                    blue: c_next(Color::Blue),
                    white: c_next(Color::White),
                }
            } else {
                let c_num = |c: Color| -> usize {
                    s[0].iter().filter(|c2| c2.as_ref().map_or(false, |c2| c == *c2)).count()
                };
                ColorIndexed{
                    red: 5 - c_num(Color::Red),
                    blue: 5 - c_num(Color::Blue),
                    white: 5 - c_num(Color::White),
                }
            }
        });
    }

    let &ColorIndexed{red, blue, white} = dp.last().unwrap();
    *(vec![red, blue, white].iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_test_1(){
        let f = |vec: Vec<String>| -> Vec<Vec<Option<Color>>> {
            vec.into_iter().map(|str| str.chars().map(|c|{
                match c {
                    'R' => Some(Color::Red),
                    'B' => Some(Color::Blue),
                    'W' => Some(Color::White),
                    '#' => None,
                    _ => unreachable!()
                }
            }).collect()).collect()
        };
        let s = vec![
            String::from("BR#WB"),
        ];
        assert_eq!(minimize(&f(s)), 3);
        let s = vec![
            String::from("W#B#R"),
            String::from("WRW#B"),
            String::from("RW#BR"),
        ];
        assert_eq!(minimize(&f(s)), 10)
    }
}
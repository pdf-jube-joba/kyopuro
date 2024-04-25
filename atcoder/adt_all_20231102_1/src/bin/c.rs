use proconio::marker::Chars;

const L: usize = 8;

fn main() {
    proconio::input! {
        s: [Chars; L],
    }
    println!("{}", chessboard(s))
}

fn chessboard(s: Vec<Vec<char>>) -> String {
    for i in 0..L {
        for j in 0..L {
            if s[i][j] == '*' {
                let firstchar: char = char::from_u32('a' as u32 + j as u32).unwrap();
                let secondchar: char = char::from_u32('1' as u32 + (7 - i) as u32).unwrap();
                return format!("{}{}", firstchar, secondchar);
            }
        }
    }
    unreachable!()
}

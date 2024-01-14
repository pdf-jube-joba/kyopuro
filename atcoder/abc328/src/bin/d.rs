use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    println!("{}", final_string(s).into_iter().collect::<String>())
}

fn final_string(s: Vec<char>) -> Vec<char> {
    let mut v = vec![];
    // if `for` goes around s[i], v becomes string when operating s[0..i] 
    for si in s {
        v.push(si);
        if v.len() >= 3 && v[v.len() - 3..v.len()] == ['A', 'B', 'C']  {
            v.pop();
            v.pop();
            v.pop();
        }
    }
    v
}

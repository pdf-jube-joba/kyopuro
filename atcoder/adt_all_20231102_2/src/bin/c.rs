use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        st: [(Chars, Chars); n],
    }
    println!("{}", if same_name(st) {"Yes"} else {"No"})
}

fn same_name(st: Vec<(Vec<char>, Vec<char>)>) -> bool {
    let n = st.len();
    for i in 0..n {
        for j in i+1.. n {
            if st[i].0 == st[j].0 && st[i].1 == st[j].1 {
                return true;
            }
        }
    }
    return false;
}

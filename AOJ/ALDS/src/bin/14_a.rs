fn find_sub(t: &[u8], p: &[u8]) -> Vec<usize> {
    let p_len = p.len();
    t.iter().enumerate().filter_map(|(i, _)|{
        if i+p_len <= t.len() && &t[i..i+p_len] == p {
            Some(i)
        } else {
            None
        }
    }).collect()
}

fn main() {
    let (t, p) = input();
    for i in find_sub(&t, &p) {
        println!("{}", i);
    }
}

fn input() -> (Vec<u8>, Vec<u8>) {
    use std::convert::TryFrom;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim().to_string().into_bytes();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let p = buf.trim().to_string().into_bytes(); 

    (t , p)
}
fn main() {
    let (t, pi) = input();
}

fn input() -> (Vec<u8>, Vec<Vec<u8>>) {
    use std::io::BufRead;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();


    let t = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().as_bytes().to_vec()
    };

    let q = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    let pi = (0..q).map(|_|{
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().as_bytes().to_vec()
    }).collect();

    (t, pi)

}
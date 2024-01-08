use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        s: Chars,
    }
    let s: Vec<usize> = s.into_iter().map(|char| match char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unimplemented!(),
    }).collect();

    println!("{}", num(m, s));
}

fn num(m: usize, s: Vec<usize>) -> usize {
    s.split(|event| *event == 0).map(|inter|{
        let event_1_num = inter.iter().filter(|event| **event == 1).count();
        let event_2_num = inter.iter().filter(|event| **event == 2).count();
        (if event_1_num <= m {0} else {event_1_num - m}) + event_2_num
    }).max().unwrap()
}

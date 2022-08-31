fn main() {
    proconio::input!{
        s: String,
    }

    println!("{}", count(&s));

}

fn count(s: &str) -> usize {
    let b: Vec<bool> = s.chars().into_iter().map(|c|{
        c == 'A' || c == 'T' || c == 'G' || c == 'C'
    }).collect();
    let mut count = 0;
    let mut max = count;
    b.iter().for_each(|b|{
        if *b {count += 1;} else {count = 0;}
        if count > max {max = count;}
    });
    max
}

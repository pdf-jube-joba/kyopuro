fn main() {
    proconio::input! {
        s1: String,
        s2: String,
        s3: String,
    }
    let input_all = vec![s1, s2, s3];
    let all = vec!["ABC", "ARC", "AGC", "AHC"];
    let t = all
        .into_iter()
        .find(|&str| !input_all.contains(&str.to_string())).unwrap();
    println!("{}", t)
}

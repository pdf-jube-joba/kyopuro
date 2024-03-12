fn main() {
    proconio::input!{
        s: String,
    }
    let b = matches!(s.as_str(), "ACE" | "BDF" | "CEG" | "DFA" | "EGB"|"FAC"|"GBD");
    println!("{}", if b {"Yes"} else {"No"})
}

use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
    }
    let (mut ks, mut qs, mut rs, mut bs, mut ns) = (vec![], vec![], vec![], vec![], vec![]);
    for (i, si) in s.into_iter().enumerate() {
        match si {
            'K' => ks.push(i),
            'Q' => qs.push(i),
            'R' => rs.push(i),
            'B' => bs.push(i),
            'N' => ns.push(i),
            _ => unreachable!(),
        }
    }
    let b = (bs[0] % 2 != bs[1] % 2) && (rs[0] < ks[0] && ks[0] < rs[1]);
    println!("{}", if b {"Yes"} else {"No"})
}

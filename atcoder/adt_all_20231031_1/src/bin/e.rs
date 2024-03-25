use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
        t: Chars,
    }
    println!("{}", if atcoder_cards(s, t) { "Yes" } else { "No" })
}

fn a2u(c: char) -> Option<usize> {
    (c as usize).checked_sub('a' as usize)
}

fn atcoder_cards(s: Vec<char>, t: Vec<char>) -> bool {
    let n = s.len();
    let (mut ams, mut amt): (usize, usize) = (0, 0);
    let mut nums: [usize; 26] = [0; 26];
    let mut numt: [usize; 26] = [0; 26];
    for i in 0..n {
        if s[i] == '@' {
            ams += 1;
        } else {
            nums[a2u(s[i]).unwrap()] += 1;
        }
        if t[i] == '@' {
            amt += 1;
        } else {
            numt[a2u(t[i]).unwrap()] += 1;
        }
    }

    for c in 'a'..='z' {
        let (ns, nt) = (nums[a2u(c).unwrap()], numt[a2u(c).unwrap()]);
        if "atcoder".contains(c) {
            if ns < nt {
                match ams.checked_sub(nt - ns) {
                    Some(v) => ams = v,
                    None => {
                        return false;
                    }
                };
            } else {
                match amt.checked_sub(ns - nt) {
                    Some(v) => amt = v,
                    None => {
                        return false;
                    }
                };
            }
        } else if ns != nt {
            return false;
        }
    }

    true
}

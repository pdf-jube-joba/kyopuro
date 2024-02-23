use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: [Chars; n],
    }
    println!("{}", if is_first_win(s) { "First" } else { "Second" })
}

fn is_first_win(s: Vec<Vec<char>>) -> bool {
    let n = s.len();
    // game_tree[c: char][S subset {0..n}] = win first when the available string is {s[i] | i in S} and start with c
    let mut game_tree: Vec<Vec<Option<bool>>> = vec![vec![]; 1 << 8];
    for c in 'a'..='z' {
        game_tree[c as usize].extend(vec![None; 1 << n]);
    }

    ('a'..='z').any(|c| is_first_win_rec(&s, &mut game_tree, c, (1 << n) - 1))
}

fn is_first_win_rec(
    s: &Vec<Vec<char>>,
    game_tree: &mut Vec<Vec<Option<bool>>>,
    c: char,
    subset: usize,
) -> bool {
    let n = s.len();
    debug_assert!(subset < (1 << n));

    if let Some(b) = game_tree[c as usize][subset] {
        return b;
    }

    let res = (0..n)
        .filter(|i| subset & (1 << i) != 0) //  i s.t. i in S
        .any(|i| {
            let (fst, lst) = (*s[i].first().unwrap(), *s[i].last().unwrap());
            fst == c && !is_first_win_rec(s, game_tree, lst, subset - (1 << i))
        });

    game_tree[c as usize][subset] = Some(res);
    res
}

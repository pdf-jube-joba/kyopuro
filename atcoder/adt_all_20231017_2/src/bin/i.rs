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
    // game_tree[S subset {0..n}][c: char] = win first when used S, last c
    let mut game_tree: Vec<Vec<Option<bool>>> = vec![vec![None; 1 << n]; 1 << 8];

    rec(n, &mut game_tree, (1 << n) - 1)
}

fn rec(n: usize, game_tree: &mut Vec<Vec<Option<bool>>>, subset: usize, c: char) -> bool {
    debug_assert!(subset < (1 << n));
    if let Some(b) = game_tree[subset][c as usize] {
        return b;
    }
    (0..n).filter_map(|i| if subset & (1 << i) != 0 {
        Some(subset - (1 << i))
    } else {
        None
    }).map(|subofsub|{

    })
    todo!()
}

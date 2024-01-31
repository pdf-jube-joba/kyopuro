use itertools::Itertools;

const ALL: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

fn main() {
    proconio::input! {
        c: [[usize; 3]; 3]
    }
    let no_gakkari_num = num_no_gakkari(c);
    println!(
        "{}",
        (no_gakkari_num as f64) / (9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1) as f64
    )
}

fn num_no_gakkari(c: Vec<Vec<usize>>) -> usize {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    v.into_iter()
        // 9!
        .permutations(9)
        .filter(|v| {
            let s = {
                let mut s = vec![vec![0; 3]; 3];
                for num in 0..9 {
                    s[v[num] / 3][v[num] % 3] = num;
                }
                s
            };

            ALL.iter().all(|v| {
                let written_num = v.iter().map(|(i, j)| c[*i][*j]).collect_vec();
                let order = v.iter().map(|(i, j)| s[*i][*j]).collect_vec();
                if written_num[0] == written_num[1] {
                    order[0] > order[2] || order[1] > order[2]
                } else if written_num[1] == written_num[2] {
                    order[1] > order[0] || order[2] > order[0]
                } else if written_num[2] == written_num[0] {
                    order[2] > order[1] || order[0] > order[1]
                } else {
                    true
                }
                // !is_gakkari(&c_r, &s_r)
            })
        })
        .count()
}

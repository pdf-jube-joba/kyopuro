use std::collections::VecDeque;

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        abxy: [(Usize1, Usize1, isize, isize); m],
    }

    let res = relative_position(n, abxy);

    for i in res {
        println!(
            "{}",
            if let Some(pos) = i {
                format!("{} {}", pos.0, pos.1)
            } else {
                "undecidable".to_string()
            }
        );
    }
}

fn relative_position(
    n: usize,
    abxy: Vec<(usize, usize, isize, isize)>,
) -> Vec<Option<(isize, isize)>> {
    // to_edges[i] = { (j, xi, yi) | (i, j, xi, yi) in abxy }
    let to_edges: Vec<Vec<(usize, (isize, isize))>> = {
        let mut to_edges = vec![vec![]; n];
        for (a, b, x, y) in abxy {
            to_edges[a].push((b, (x, y)));
            to_edges[b].push((a, (-x, -y)))
        }
        to_edges
    };

    //
    let mut position: Vec<Option<(isize, isize)>> = vec![None; n];

    let mut queue: VecDeque<(usize, (isize, isize))> = VecDeque::new();
    queue.push_back((0, (0, 0)));

    while let Some((next, pos)) = queue.pop_front() {
        if let Some(pos_prev) = position[next] {
            if pos_prev != pos {
                unreachable!("情報が矛盾している！");
            }
        } else {
            position[next] = Some(pos);
            for &(neighbor, (x, y)) in &to_edges[next] {
                queue.push_back((neighbor, (pos.0 + x, pos.1 + y)));
            }
        }
    }

    position
}

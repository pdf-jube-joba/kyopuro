use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};

use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use rand::Rng;

const DIRECTION: [(isize, isize); 4] = [(-1, 1), (1, 1), (-1, -1), (1, -1)];

fn random_input(size: usize) -> ((usize, usize), (usize, usize), Vec<Vec<bool>>) {
    let mut s: Vec<Vec<bool>> = vec![];
    let (s, c) = loop {
        s = (0..size)
            .map(|_| (0..size).map(|_| rand::random()).collect())
            .collect();
        let c = iproduct!(0..size, 0..size)
            .filter(|&(i, j)| !s[i][j])
            .collect_vec();
        if c.len() >= 2 {
            break (s, c);
        }
    };
    let mut rng = rand::thread_rng();
    let ai = rng.gen_range(0, c.len());
    let bi = loop {
        let bi = rng.gen_range(0, c.len());
        if bi != ai {
            break bi;
        }
    };
    (c[ai], c[bi], s)
}

fn worst(size: usize) -> ((usize, usize), (usize, usize), Vec<Vec<bool>>) {
    let mut s = vec![vec![false; size]; size];
    let a = (0, 0);
    let b = (size - 1, size - 1);
    s[size - 2][size - 2] = true;
    (a, b, s)
}

fn move_to(n: usize, from: (usize, usize), diff: (isize, isize)) -> Option<(usize, usize)> {
    let t = (from.0 as isize + diff.0, from.1 as isize + diff.1);
    if 0 <= t.0 && t.0 < n as isize && 0 <= t.1 && t.1 < n as isize {
        Some((t.0 as usize, t.1 as usize))
    } else {
        None
    }
}

fn bishop2_1(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();
    let mut visited: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((a, 0));
    while let Some((next, step)) = queue.pop_front() {
        if visited[next.0][next.1].is_some() {
            continue;
        }
        visited[next.0][next.1] = Some(step);
        for dir in DIRECTION {
            for d in 0..n {
                let Some(t) = move_to(n, next, (d as isize * dir.0, d as isize * dir.1)) else {
                    break;
                };
                if s[t.0][t.1] {
                    break;
                }
                queue.push_back((t, step + 1));
            }
        }
    }
    visited[b.0][b.1]
}

fn bishop2_2(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();

    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![std::usize::MAX; DIRECTION.len()]; n]; n];
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; DIRECTION.len()]; n]; n];

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    for i in 0..DIRECTION.len() {
        let direction = DIRECTION[i];
        let Some(next) = move_to(n, a, direction) else {
            continue;
        };
        if s[next.0][next.1] {
            continue;
        }
        dist[next.0][next.1][i] = 1;
        queue.push_back((next, i));
    }

    while let Some((next, next_i)) = queue.pop_front() {
        if visited[next.0][next.1][next_i] {
            continue;
        }
        visited[next.0][next.1][next_i] = true;

        if next == b {
            return Some(dist[next.0][next.1][next_i]);
        }

        let dist_next = dist[next.0][next.1][next_i];

        for neigh_i in 0..DIRECTION.len() {
            let Some(neigh) = move_to(n, next, DIRECTION[neigh_i]) else {
                continue;
            };
            if s[neigh.0][neigh.1] {
                continue;
            }

            let diff: usize = if next_i == neigh_i { 0 } else { 1 };

            if dist[neigh.0][neigh.1][neigh_i] <= dist_next + diff {
                continue;
            }

            dist[neigh.0][neigh.1][neigh_i] = dist_next + diff;

            if next_i == neigh_i {
                queue.push_front((neigh, neigh_i));
            } else {
                queue.push_back((neigh, neigh_i));
            }
        }
    }

    None
}

fn bishop2_3(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();

    let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; n];
    dist[a.0][a.1] = 0;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(a);

    while let Some(next) = queue.pop_front() {
        for direction in DIRECTION {
            let mut count = 0;
            loop {
                count += 1;
                let Some(n) = move_to(n, next, (direction.0 * count, direction.1 * count)) else {
                    break;
                };
                if s[n.0][n.1] {
                    break;
                }
                if dist[n.0][n.1] == dist[next.0][next.1] + 1 {
                    continue;
                    // break; ではだめらしい。
                }
                if dist[n.0][n.1] != std::usize::MAX {
                    break;
                }
                dist[n.0][n.1] = dist[next.0][next.1] + 1;
                queue.push_back(n);
            }
        }
    }

    if dist[b.0][b.1] == std::usize::MAX {
        None
    } else {
        Some(dist[b.0][b.1])
    }
}

fn bench_comps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("bishop2");
    group.plot_config(plot_config);
    for size in 5..10 {
        let input = worst(size);
        group.bench_with_input(BenchmarkId::new("bishop2 1", size), &input, |b, grid| {
            b.iter(|| bishop2_1(input.0, input.1, input.2.clone()))
        });
        group.bench_with_input(BenchmarkId::new("bishop2 2", size), &input, |b, grid| {
            b.iter(|| bishop2_2(input.0, input.1, input.2.clone()))
        });
        group.bench_with_input(BenchmarkId::new("bishop2 3", size), &input, |b, grid| {
            b.iter(|| bishop2_3(input.0, input.1, input.2.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_comps);
criterion_main!(benches);

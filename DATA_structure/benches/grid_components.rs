use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration, AxisScale};
use itertools::iproduct;
use std::collections::VecDeque;

fn adj((h, w): (usize, usize), (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    vec![
        if i > 0 { Some((i - 1, j)) } else { None },
        if j > 0 { Some((i, j - 1)) } else { None },
        if j < w - 1 { Some((i, j + 1)) } else { None },
        if i < h - 1 { Some((i + 1, j)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn queue_non_filter(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_front() {
            if !grid[pt.0][pt.1] || grid_components[pt.0][pt.1].is_some() {
                continue;
            }
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(adj((h, w), pt));
        }

        comp_num += 1;
    }

    grid_components
}

fn queue_filter(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_front() {
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(
                adj((h, w), pt)
                    .filter(|pt2| grid_components[pt2.0][pt2.1].is_none() && grid[pt2.0][pt2.1]),
            );
        }

        comp_num += 1;
    }

    grid_components
}

fn stack_non_filter(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_back() {
            if !grid[pt.0][pt.1] || grid_components[pt.0][pt.1].is_some() {
                continue;
            }
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(adj((h, w), pt));
        }

        comp_num += 1;
    }

    grid_components
}

fn stack_filter(grid: Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let (h, w) = (grid.len(), grid[0].len());

    let mut grid_components: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    let mut comp_num = 0;

    for (i, j) in iproduct!(0..h, 0..w) {
        if grid_components[i][j].is_some() || !grid[i][j] {
            continue;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from_iter(vec![(i, j)]);

        while let Some(pt) = queue.pop_back() {
            grid_components[pt.0][pt.1] = Some(comp_num);
            queue.extend(
                adj((h, w), pt)
                    .filter(|pt2| grid_components[pt2.0][pt2.1].is_none() && grid[pt2.0][pt2.1]),
            );
        }

        comp_num += 1;
    }

    grid_components
}

fn bench_comps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Components computation");
    group.plot_config(plot_config);
    for size in 1..10 {
        let grid = vec![vec![true; size]; size];
        group.bench_with_input(
            BenchmarkId::new("queue non filter", size),
            &grid,
            |b, grid| b.iter(|| queue_non_filter(grid.clone())),
        );
        group.bench_with_input(BenchmarkId::new("queue filter", size), &grid, |b, grid| {
            b.iter(|| queue_filter(grid.clone()))
        });
        group.bench_with_input(
            BenchmarkId::new("stack non filter", size),
            &grid,
            |b, grid| b.iter(|| stack_non_filter(grid.clone())),
        );
        group.bench_with_input(BenchmarkId::new("stack filter", size), &grid, |b, grid| {
            b.iter(|| stack_filter(grid.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_comps);
criterion_main!(benches);

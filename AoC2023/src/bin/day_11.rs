fn main() {
    let input = input();
    println!("{}", compute_part1(input.clone()));
    println!("{}", compute_part2(input));
}

fn compute_part1(input: Vec<Vec<Image>>) -> usize {
    let stretched = stretch(input);
    let mut all_gal = vec![];
    for i in 0..stretched.len() {
        for j in 0..stretched[0].len() {
            if stretched[i][j] == Image::Galaxy {
                all_gal.push((i, j));
            }
        }
    }
    let mut sum = 0;
    for pt1 in &all_gal {
        for pt2 in &all_gal {
            let diff = pt1.0.abs_diff(pt2.0) + pt1.1.abs_diff(pt2.1);
            sum += diff;
        }
    }
    sum / 2
}

fn compute_part2(input: Vec<Vec<Image>>) -> usize {
    let h = input.len();
    let w = input[0].len();
    let all_space_row: Vec<usize> = (0..h)
        .filter(|&i| input[i].iter().all(|img| *img == Image::Space))
        .collect();
    let all_space_col: Vec<usize> = (0..w)
        .filter(|&i| input.iter().all(|row| row[i] == Image::Space))
        .collect();
    let all_gal: Vec<_> = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .filter(|&(i, j)| input[i][j] == Image::Galaxy)
        .collect();
    let sum: usize = all_gal
        .iter()
        .flat_map(|pt1| all_gal.iter().map(move |pt2| (pt1, pt2)))
        .map(|(&pt1, &pt2)| compute_gal_pathlen(pt1, pt2, &all_space_row, &all_space_col))
        .sum();
    sum / 2
}

fn compute_gal_pathlen(
    pt1: (usize, usize),
    pt2: (usize, usize),
    all_space_row: &Vec<usize>,
    all_space_col: &Vec<usize>,
) -> usize {
    let pt1_row_ind = all_space_row.binary_search(&pt1.0).unwrap_err();
    let pt1_col_ind = all_space_col.binary_search(&pt1.1).unwrap_err();
    let pt2_row_ind = all_space_row.binary_search(&pt2.0).unwrap_err();
    let pt2_col_ind = all_space_col.binary_search(&pt2.1).unwrap_err();
    let num_sp_row = pt1_row_ind.abs_diff(pt2_row_ind);
    let num_sp_col = pt1_col_ind.abs_diff(pt2_col_ind);
    (pt1.0.abs_diff(pt2.0) - num_sp_row)
        + num_sp_row * 1_000_000
        + (pt1.1.abs_diff(pt2.1) - num_sp_col)
        + num_sp_col * 1_000_000
}

fn num_of_range((start, end): (usize, usize), sorted: &Vec<usize>) -> usize {
    if end < start {
        return num_of_range((end, start), sorted);
    }
    let s_ind = sorted.binary_search(&start).unwrap_err();
    let e_ind = sorted.binary_search(&end).unwrap_err();
    e_ind - s_ind
}

fn stretch(v: Vec<Vec<Image>>) -> Vec<Vec<Image>> {
    let mut a = vec![];
    let col_emp_nums = (0..v[0].len())
        .filter(|i| v.iter().all(|v_row| v_row[*i] == Image::Space))
        .collect::<Vec<_>>();
    let after_row_len = v[0].len() + col_emp_nums.len();
    for v_row in v {
        let mut a_row = vec![];
        if v_row.iter().all(|img| *img == Image::Space) {
            a.push(vec![Image::Space; after_row_len]);
        }
        for (j, img) in v_row.into_iter().enumerate() {
            if col_emp_nums.contains(&j) {
                a_row.push(Image::Space);
            }
            a_row.push(img);
        }
        a.push(a_row);
    }
    a
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Image {
    Space,
    Galaxy,
}

fn input() -> Vec<Vec<Image>> {
    let string = std::fs::read_to_string("inout/day11.in").unwrap();
    let mut v = vec![];
    for line in string.lines() {
        v.push(
            line.chars()
                .map(|char| match char {
                    '.' => Image::Space,
                    '#' => Image::Galaxy,
                    _ => unreachable!("?? {char}"),
                })
                .collect::<Vec<_>>(),
        );
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stretch_test() {
        let v = vec![vec![Image::Galaxy]];
        assert_eq!(stretch(v), vec![vec![Image::Galaxy]]);

        let v = vec![vec![Image::Space]];
        assert_eq!(
            stretch(v),
            vec![
                vec![Image::Space, Image::Space],
                vec![Image::Space, Image::Space]
            ]
        );

        let v = vec![vec![Image::Galaxy, Image::Space]];
        assert_eq!(
            stretch(v),
            vec![vec![Image::Galaxy, Image::Space, Image::Space]]
        );

        let v = vec![vec![Image::Galaxy], vec![Image::Space]];
        assert_eq!(
            stretch(v),
            vec![vec![Image::Galaxy], vec![Image::Space], vec![Image::Space]]
        );
    }
    #[test]
    fn gal_diff_test() {
        let all_space_row = vec![2, 4, 6];
        let all_space_col = vec![3, 5, 7];
        assert_eq!(
            compute_gal_pathlen((0, 0), (1, 1), &all_space_row, &all_space_col),
            2
        );
        assert_eq!(
            compute_gal_pathlen((1, 1), (0, 0), &all_space_row, &all_space_col),
            2
        );
        assert_eq!(
            compute_gal_pathlen((1, 1), (3, 1), &all_space_row, &all_space_col),
            (3 - 1) - 1 + 1_000_000
        );
        assert_eq!(
            compute_gal_pathlen((1, 2), (7, 8), &all_space_row, &all_space_col),
            1_000_000 * 6 + 6
        );
    }
}

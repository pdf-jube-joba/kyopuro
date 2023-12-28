fn main() {
    let i = input();
}

enum Direction {
    Up, Down, Right, Left,
}

fn compute_part1(input: &Vec<Vec<usize>>) -> usize {
    let h = input.len();
    let w = input[0].len();
    // vertex of graph = {(i,j,d) | 0 <= i < h, 0 <= j < w, d \in {U,D,R,L}}
    // edge of graph = { (i1,j1,d1), (i2,j2,d2) \in (vertex, vertex) | (i2, j2) is in d1 direction from (i1, j1) and dist <= 3 && ... } 
    todo!()
}

fn input() -> Vec<Vec<usize>> {
    let string = std::fs::read_to_string("inout/day17.in").unwrap();
    string
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| match char.to_digit(10) {
                    Some(digit) => digit as usize,
                    None => unreachable!("{}", char),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

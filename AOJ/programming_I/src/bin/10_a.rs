fn main() {
    let (p1, p2) = input();
    println!("{}", abs(p1, p2));
}

fn abs(p1: Point, p2: Point) -> f64 {
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

struct Point {
    x: f64,
    y: f64,
}

fn input() -> (Point, Point) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let v: Vec<f64> = string.split_whitespace().map(|str| str.parse::<f64>().unwrap()).collect();
    (
        Point {
            x: v[0],
            y: v[1],
        },
        Point {
            x: v[2],
            y: v[3],
        }
    )
}
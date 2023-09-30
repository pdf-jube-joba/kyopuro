fn main() {
    let n = input();
    let ps = line_rec(
        Point { x: 0_f64, y: 0_f64 },
        Point { x: 100_f64, y: 0_f64 },
        n
    );
    for p in ps {
        println!("{} {}", p.x, p.y);
    }
}

fn line_rec(
    p1: Point,
    p2: Point,
    n: usize,
) -> Vec<Point> {
    if n == 0 {
        vec![p1, p2]
    } else {
        let (p1, s, u, t, p2) = line_split(p1, p2);
        let mut v = vec![p1];
        v.extend(&line_rec(p1, s, n - 1)[1..]);
        v.extend(&line_rec(s, u, n - 1)[1..]);
        v.extend(&line_rec(u, t, n - 1)[1..]);
        v.extend(&line_rec(t, p2, n - 1)[1..]);
        v
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> <Self as std::ops::Add<Self>>::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn scalar(a: f64, p: Point) -> Point {
    Point { x: a * p.x, y: a * p.y }
}

fn rotate(r: f64, p: Point) -> Point {
    Point { 
        x:  r.cos() * p.x - r.sin() * p.y,
        y: r.sin() * p.x + r.cos() * p.y,
    }
}

fn line_split(
    p1: Point,
    p2: Point
) -> (Point, Point, Point, Point, Point) {
    let co = scalar(1_f64/  3_f64, p2 + scalar(-1_f64, p1));
    (
        p1,
        co + p1,
        rotate(std::f64::consts::PI / 3_f64, co) + co + p1,
        scalar(2_f64, co) + p1,
        p2,
    )
}

fn input() -> usize {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string.trim().parse::<usize>().unwrap()
}
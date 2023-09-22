fn main() {
    let (w, h, x, y, r) = input();
    let width_is_ok = 0 <= x - (r as isize) && x + (r as isize) <= (w as isize);
    let height_is_ok = 0 <= y - (r as isize) && y + (r as isize) <= (h as isize);
    if width_is_ok && height_is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn input() -> (usize, usize, isize, isize, usize) {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let vec: Vec<&str> = string.split_whitespace().collect();
    let w = vec[0].parse::<usize>().unwrap();
    let h = vec[1].parse::<usize>().unwrap();
    let x = vec[2].parse::<isize>().unwrap();
    let y = vec[3].parse::<isize>().unwrap();
    let r = vec[4].parse::<usize>().unwrap();
    (w, h, x, y, r)
}
fn input() -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let tuple: Vec<usize> = str
            .split_whitespace()
            .map(|str|{str.parse().unwrap()})
            .collect();
        let (x,y) = (tuple[0], tuple[1]);
        if x == 0 && y == 0 {
            break
        } else {
            vec.push((x, y));
        }
    }
    vec
}
fn main() {

    let vec = input();
    
    for (n, x) in vec {
        println!("{}", count(n, x));
    }

}

fn count(n: usize, x: usize) -> usize {
    (1..n)
    .flat_map(|i| (i+1..n).map(move |j|(i,j)))
    .filter(|&(i,j)| {
        i + j < x && {
            let k = x - (i + j);
            j < k && k <= n
        }
    })
    .count()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_count() {
        assert_eq!(count(5,9), 2);
        assert_eq!(count(10,0), 0);
        assert_eq!(count(3,10), 0);
    }
}
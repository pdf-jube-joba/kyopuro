#[derive(Debug, Clone)]
struct Heap<T> {
    vec: Vec<T>,
}

impl<T> Heap<T> {
    fn new() -> Self {
        Heap { vec: vec![] }
    }
    fn push(&mut self, value: T) {
        self.vec.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    fn get_by_one_index(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return None;
        }
        self.vec.get(index - 1)
    }
    fn parent(&self, index: usize) -> Option<&T> {
        self.get_by_one_index(index / 2)
    }
    fn left(&self, index: usize) -> Option<&T> {
        self.get_by_one_index(2 * index)
    }
    fn right(&self, index: usize) -> Option<&T> {
        self.get_by_one_index(2 * index + 1)
    }
}

fn main() {
    let input = input();
    let h = input.len();
    let mut heap = Heap::new();
    for value in input {
        heap.push(value);
    }

    for i in 1..=h {
        println!(
            "node {}: {}{}{}{}",
            i,
            format!("key = {}, ", heap.get_by_one_index(i).unwrap()),
            {
                match heap.parent(i) {
                    None => "".to_owned(),
                    Some(id) => format!("parent key = {}, ", id),
                }
            },
            {
                match heap.left(i) {
                    None => "".to_owned(),
                    Some(id) => format!("left key = {}, ", id),
                }
            },
            {
                match heap.right(i) {
                    None => "".to_owned(),
                    Some(id) => format!("right key = {}, ", id),
                }
            },
        );
    }
}

fn input() -> Vec<isize> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _h = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|str| str.parse::<isize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heap_test() {
        let mut heap = Heap::new();
        heap.push(1);
        heap.pop();


        heap.push(2);
        assert_eq!(heap.get_by_one_index(1), Some(&2));

        heap.push(4);
        heap.push(6);
        
        assert_eq!(heap.left(1), Some(&4));
    }
}

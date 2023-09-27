fn main() {
    let (q, process) = input();
    let mut queue: QueueRound<(String, usize)> = QueueRound::new(process);
    let mut now = 0;
    loop {
        match queue.pop_front() {
            None => {
                break;
            }
            Some(mut process) => {
                if process.1 <= q {
                    now += process.1;
                    println!("{} {}", process.0, now);
                } else {
                    now += q;
                    process.1 -= q;
                    queue.push_back(process).unwrap();
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct QueueRound<T> {
    vec: Vec<Option<T>>,
    head: usize,
    tail: usize,
    is_empty: bool,
}

fn mod_plus_1(a: &mut usize, m: usize){
    *a = (*a + 1) % m;
}

fn mod_minus_1(a: &mut usize, m: usize) {
    if *a == 0 {
        *a = m - 1;
    } else {
        *a -= 1;
    }
}

impl<T> QueueRound<T> {
    fn capacity(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.is_empty
    }
    fn new(vec: Vec<T>) -> Self {
        let len = vec.len();
        let is_empty = vec.is_empty();
        Self {
            vec: vec.into_iter().map(Some).collect(),
            head: 0,
            tail: len - 1,
            is_empty,
        }
    }
    fn push_front(&mut self, elem: T) -> Result<(), T> {
        if self.is_empty() {
            self.vec[self.head].replace(elem);
            self.is_empty = false;
            Ok(())
        } else {
            let cap = self.capacity();
            mod_minus_1(&mut self.head, cap);
            if self.vec[self.head].is_some() {
                mod_plus_1(&mut self.head, cap);
                return Err(elem);
            }
            self.vec[self.head].replace(elem);
            self.is_empty = false;
            Ok(())
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        let elem = self.vec[self.head].take();
        while self.vec[self.head].is_none() && self.head != self.tail {
            let cap = self.capacity();
            mod_plus_1(&mut self.head, cap);
        }
        self.is_empty = self.vec[self.head].is_none();
        elem
    }
    fn push_back(&mut self, elem: T) -> Result<(), T> {
        if self.is_empty() {
            self.vec[self.tail].replace(elem);
            self.is_empty = false;
            Ok(())
        } else {
            let cap = self.capacity();
            mod_plus_1(&mut self.tail, cap);
            if self.vec[self.tail].is_some() {
                mod_minus_1(&mut self.tail, cap);
                return Err(elem);
            }
            self.vec[self.tail].replace(elem);
            self.is_empty = false;
            Ok(())
        }
    }
    fn pop_back(&mut self) -> Option<T> {
        let elem = self.vec[self.tail].take();
        while self.vec[self.tail].is_none() && self.head != self.tail {
            let cap = self.capacity();
            mod_minus_1(&mut self.tail, cap);
        }
        self.is_empty = self.vec[self.tail].is_none();
        elem
    }
}

fn input() -> (usize, Vec<(String, usize)>) {
    let stdin = std::io::stdin();
    let mut string = String::new();
    stdin.read_line(&mut string).unwrap();
    let (n ,q) = {
        let mut splitted = string.split_whitespace();
        (splitted.next().unwrap().parse::<usize>().unwrap(),
        splitted.next().unwrap().parse::<usize>().unwrap())
    };

    string = String::new();
    let v: Vec<_> = (0..n).map(|_|{
        string = String::new();
        stdin.read_line(&mut string).unwrap();
        let mut splitted = string.split_whitespace();
        let first = splitted.next().unwrap();
        let second = splitted.next().unwrap().parse::<usize>().unwrap();
        (first.to_owned(), second)
    }).collect();
    (q, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_round_test(){
        let mut queue: QueueRound<usize> = QueueRound::new(vec![1,2,3]);
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_front(), Some(2));
        queue.push_front(2).unwrap();
        assert_eq!(queue.pop_front(), Some(2));
        assert_eq!(queue.pop_front(), Some(3));
        assert_eq!(queue.pop_front(), None);
        assert_eq!(queue.pop_front(), None);
        queue.push_front(3).unwrap();
        queue.push_front(2).unwrap();
        queue.push_front(1).unwrap();
        assert_eq!(queue.push_front(4), Err(4));

        assert_eq!(queue.pop_back(), Some(3));
        assert_eq!(queue.pop_back(), Some(2));
        assert_eq!(queue.pop_back(), Some(1));
        assert_eq!(queue.pop_back(), None);

        eprintln!("{:?}", queue);
        queue.push_front(3).unwrap();     
        eprintln!("{:?}", queue);
        queue.push_front(2).unwrap(); 
        eprintln!("{:?}", queue);
        queue.push_front(1).unwrap();         eprintln!("{:?}", queue);
        assert_eq!(queue.push_back(4), Err(4));

        assert_eq!(queue.pop_front(), Some(1));
        queue.push_back(1).unwrap();
        eprintln!("{:?}", queue);
    }
}
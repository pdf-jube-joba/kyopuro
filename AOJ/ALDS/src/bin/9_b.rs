#[derive(Debug, Clone)]
struct MaxHeap<T: PartialOrd + Ord> {
    vec: Vec<T>,
}

impl<T> MaxHeap<T> where T: PartialOrd + Ord + std::fmt::Debug + Clone {
    fn swap(&mut self, i: usize, j: usize) {
        if i > 0 && j > 0 {
            self.vec.swap(i - 1, j - 1);
        }
    }
     fn capacity(&self) -> usize {
        self.vec.len()
    }
    fn new() -> Self {
        MaxHeap { vec: vec![] }
    }
    fn push(&mut self, value: T) {
        self.vec.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return None;
        }
        self.vec.get(index - 1)
    }
    fn parent(&self, index: usize) -> Option<&T> {
        self.get(index / 2)
    }
    fn left(&self, index: usize) -> Option<&T> {
        self.get(2 * index)
    }
    fn right(&self, index: usize) -> Option<&T> {
        self.get(2 * index + 1)
    }
    fn max_heapify(&mut self, index: usize) {
        let l = 2 * index; // left(i)
        let r = 2 * index + 1; // right(i)

        let largest = {
            let mut largest = if 
                l <= self.capacity() &&
                self.get(l).unwrap() > self.get(index).unwrap() {
                l
            } else {
                index
            };
            if r <= self.capacity() &&
            self.get(r).unwrap() > self.get(largest).unwrap() {
                largest = r;
            }
            largest
        };
        
        if largest != index {
            self.swap(index, largest);
            self.max_heapify(largest);
        }
    }
    fn build_maxheap(&mut self) {
        let h = self.capacity();
        for i in (1..=h/2).rev() {
            self.max_heapify(i);
        }
    }
    fn collect_to_vec(&self) -> Vec<T> {
        let h = self.capacity();
        (1..=h).map(|i| self.get(i).unwrap().clone()).collect()
    }
}

fn main() {
    let input = input();
    let mut heap = MaxHeap::new();
    for value in input {
        heap.push(value);
    }
    heap.build_maxheap();

    let h = heap.capacity();

    for i in 1..=h {
        print!(" {}", heap.get(i).unwrap());
    }
    println!()
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
        let mut heap = MaxHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.collect_to_vec(), vec![1,2,3]);
        heap.build_maxheap();
        assert_eq!(heap.collect_to_vec(), vec![3,2,1]);

        let mut heap = MaxHeap::new();
        for i in 1..8 {
            heap.push(i);
        }

        assert_eq!(heap.collect_to_vec(), vec![1,2,3,4,5,6,7]);

        heap.max_heapify(3);
        heap.max_heapify(2);

        assert_eq!(heap.collect_to_vec(), vec![1,5,7,4,2,6,3]);

        eprintln!("check");
        heap.max_heapify(1);

        assert_eq!(heap.collect_to_vec(), vec![7,5,6,4,2,1,3]);

    }
}
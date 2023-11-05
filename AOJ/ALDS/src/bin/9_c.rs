#[derive(Debug, Clone)]
struct MaxHeap<T: PartialOrd + Ord> {
    vec: Vec<Option<T>>,
}

impl<T> MaxHeap<T> where T: PartialOrd + Ord + std::fmt::Debug + Clone {
    fn new() -> Self {
        MaxHeap { vec: vec![] }
    }
    fn capacity(&self) -> usize {
       self.vec.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        if i > 0 && j > 0 {
            self.vec.swap(i - 1, j - 1);
        }
    }
    fn push(&mut self, value: T) {
        self.vec.push(Some(value));
    }
    // fn pop(&mut self) -> Option<T> {
    //     self.vec.pop().flatten()
    // }
    fn pop_max(&mut self) -> Option<T> {
        let res = self.vec[0].take();
        self.build_maxheap();
        res
    }
    fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return None;
        }
        self.vec.get(index - 1).map(|opt| opt.as_ref()).flatten()
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            return None;
        }
        self.vec.get_mut(index - 1).map(|opt| opt.as_mut()).flatten()
    }
    fn take(&mut self, index: usize) -> Option<T> {
        if index == 0 || self.vec.len() <= index {
            return None;
        }
        self.vec[index].take()
    }
    fn insert(&mut self, index: usize, value: Option<T>) {
        match value {
                None => {
                    if self.get(index).is_some() {
                        self.vec[index] = None;
                    }
                }
                Some(value) => {
                    if self.get(index).is_some() {
                        self.vec[index] = Some(value)
                    }
                }
        }
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
        use std::cmp::Ordering;
        let l = 2 * index; // left(i)
        let r = 2 * index + 1; // right(i)
        let (opt_i, opt_l, opt_r) = (self.get(index), self.get(l), self.get(r));

        let largest = {
            if opt_i >= opt_l && opt_i >= opt_r {
                index
            } else if opt_l >= opt_r && opt_l >= opt_i {
                l
            } else {
                r
            }
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

enum Order {
    Insert(usize),
    Extract,
}

fn main() {
    let orders = input();
    let mut heap = MaxHeap::new();
    for order in orders {
        match order {
            Order::Insert(k) => {
                heap.push(k);
                heap.build_maxheap();
            }
            Order::Extract => {
                let poped = heap.pop_max().unwrap();
                println!("{}", poped);
            }
        }
    }
}

fn input() -> Vec<Order> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut orders = vec![];
    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let strs: Vec<&str> = buf.split_whitespace().collect();
        let order = 
        match strs[0] {
            "insert" => Order::Insert((&strs[1]).parse().unwrap()),
            "extract" => Order::Extract,
            "end" => break,
            _ => unreachable!(),
        };
        orders.push(order)
    }
    orders
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opt_test() {
        assert!(None < Some(1));
        assert!(None < Some((1, 1)));
        assert!(Some((0, 2)) < Some((1, 1)));
        assert!(Some((1, 1)) < Some((1, 2)));
    }
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
    #[test]
    fn queue_test() {
        let mut queue = MaxHeap::new();

        queue.push(3);
        assert_eq!(queue.pop_max(), Some(3));

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);
        queue.build_maxheap();

        assert_eq!(queue.pop_max(), Some(4));
        assert_eq!(queue.pop_max(), Some(3));
        assert_eq!(queue.pop_max(), Some(2));
        assert_eq!(queue.pop_max(), Some(1));

    }
}
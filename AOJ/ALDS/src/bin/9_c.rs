#[derive(Debug, Clone)]
struct MaxHeap<T: PartialOrd + Ord> {
    vec: Vec<Option<T>>,
}

fn parent(index: usize) -> Option<usize> {
    if index == 0 {
        None
    } else {
        Some((index - 1) / 2)
    }
}

fn left(index: usize) -> usize {
    2 * index + 1
}

fn right(index: usize) -> usize {
    2 * index + 2
}

impl<T> MaxHeap<T>
where
    T: PartialOrd + Ord + std::fmt::Debug + Clone,
{
    fn new() -> Self {
        MaxHeap {
            vec: Vec::with_capacity(2_000_000_000),
        }
    }
    fn is_max_heap(&self) -> bool {
        for i in 0..self.capacity() {
            if self.get(i) < self.get(left(i)) || self.get(i) < self.get(right(i)) {
                return false;
            }
        }
        true
    }
    fn capacity(&self) -> usize {
        self.vec.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.vec.swap(i, j);
    }
    fn push_align(&mut self, value: T) {
        let mut i = self.vec.len();
        self.vec.push(Some(value));
        while let Some(p) = parent(i) {
            if self.get(i) <= self.get(p) {
                break;
            }
            self.swap(i, p);
            i = p;
        }
        debug_assert!(self.is_max_heap());
    }
    fn pop_max(&mut self) -> Option<T> {
        let res = self.vec[0].take();
        self.max_heapify(0);
        res
    }
    fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index).map(|opt| opt.as_ref()).flatten()
    }
    fn max_heapify(&mut self, mut index: usize) {
        loop {
            let l = left(index);
            let r = right(index);
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

            if largest == index {
                return;
            }

            self.swap(index, largest);
            index = largest;
        }
    }
    fn build_maxheap(&mut self) {
        let h = self.capacity();
        for i in (0..h / 2).rev() {
            self.max_heapify(i);
        }
    }
    fn collect_to_vec(&self) -> Vec<T> {
        let h = self.capacity();
        (0..h).map(|i| self.get(i).unwrap().clone()).collect()
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
                heap.push_align(k);
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
        let order = match strs[0] {
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
    fn queue_test() {
        let mut queue = MaxHeap::new();

        queue.push_align(3);
        assert_eq!(queue.pop_max(), Some(3));

        queue.push_align(1);
        queue.push_align(2);
        queue.push_align(3);
        queue.push_align(4);

        assert_eq!(queue.pop_max(), Some(4));
        assert_eq!(queue.pop_max(), Some(3));
        assert_eq!(queue.pop_max(), Some(2));
        assert_eq!(queue.pop_max(), Some(1));
    }

    #[test]
    fn queue_heavy() {
        let mut queue = MaxHeap::new();
        for i in 0..1_000_0 {
            queue.push_align(i);
        }
        for i in (1_000_0..0).rev() {
            assert_eq!(queue.pop_max(), Some(i));
        }
    }
}

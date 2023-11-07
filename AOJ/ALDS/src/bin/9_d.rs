#[derive(Debug, Clone)]
struct MaxHeap<T: Ord>{
    vec: Vec<T>,
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn parent(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i-1) / 2)
    }
}

impl<T: Ord + std::fmt::Debug> MaxHeap<T> {
    fn new() -> Self {
        MaxHeap { vec: vec![] }
    }
    fn from_vec(vec: Vec<T>) -> Self {
        MaxHeap{ vec }
    }
    fn capacity(&self) -> usize {
        self.vec.len()
    }
    fn swap(&mut self, i: usize, j: usize) -> Option<()> {
        if self.capacity() <= i ||  self.capacity() <= j {
            None
        } else {
            self.vec.swap(i,j);
            Some(())
        }
    }
    fn get(&self, index: usize) -> Option<&T> {
        if self.capacity() <= index {
            None
        } else {
            self.vec.get(index)
        }
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.capacity() <= index {
            None
        } else {
            self.vec.get_mut(index)
        }
    }
    fn push_break(&mut self, value: T) {
        self.vec.push(value);
    }
    fn is_maxheap(&self) -> bool {
        let n = self.capacity();
        for i in 0..n {
            if self.get(i) < self.get(left(i)) || self.get(i) < self.get(right(i)) {
                return false;
            }
        }
        true
    }
    fn max_heapify(&mut self, i: usize, heapsize: usize) -> usize {
        let largest = {
            let l = left(i);
            let r = right(i);
            // vec![l,r,i].into_iter().max_by_key(|i| self.get(*i)).unwrap()

            let mut largest =
                if l < heapsize && self.get(l) > self.get(i) {
                    l
                } else {
                    i
                };
            if r < heapsize && self.get(r) > self.get(largest) {
                largest = r;
            }
            largest
        };

        if largest != i {
            self.swap(i, largest).unwrap();
            self.max_heapify(largest, heapsize) + 1
        } else {
            0
        }
    }
    fn build_maxheap(&mut self) {
        let n = self.capacity();
        for i in (0..n/2).rev() {
            self.max_heapify(i, n);    
        }
    }
    fn into_sort_count(mut self) -> (Vec<T>, usize) {
        self.build_maxheap();
        debug_assert!(self.is_maxheap());

        let mut heapsize = self.capacity();
        let mut count = 0;

        while heapsize >= 2 {
            eprintln!("--{:?} heapsize {:?}", self, heapsize);
            self.swap(0, heapsize - 1);
            heapsize -= 1;
            count += self.max_heapify(0, heapsize);
        }
        
        (self.vec, count)
    }
}

// return a max heap which satisfies
// 1. after take a top of tree, it costs worst times which is equal to height of tree to swapping in tree.max_heapify(0)
// 2. last element is 1
fn construct_bad_maxheap(n: usize) -> Option<MaxHeap<usize>> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(MaxHeap{
            vec: vec![1],
        });
    }
    let mut induction_maxheap = construct_bad_maxheap(n - 1)?;
    let mut i = induction_maxheap.capacity() - 1;

    debug_assert_eq!(induction_maxheap.get(i), Some(&1));

    while let Some(p) = parent(i) {
        induction_maxheap.swap(p, i);
        i = p;
    }

    *induction_maxheap.get_mut(0).unwrap() = n;
    induction_maxheap.push_break(1);

    Some(induction_maxheap)
}
 
fn main() {
    let mut a = input();
    let n = a.len();
    a.sort();

    let bad_case = construct_bad_maxheap(n).unwrap();

    println!("{}", {
        bad_case.vec.into_iter().map(|i|{
            a[i-1].to_string()
        }).collect::<Vec<_>>().join(" ")
    });

}

fn input() -> Vec<usize> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_case() {
        for i in 1..10 {
            let bad_case = construct_bad_maxheap(i).unwrap();
            eprintln!("{:?}", bad_case);
            let (v, c) = bad_case.into_sort_count();
            eprintln!("  v:{:?} c:{:?}", v, c);
        }
    }
}

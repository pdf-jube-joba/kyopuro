#[derive(Debug, Clone, Copy)]
struct Pointer(usize);

#[derive(Debug, Clone)]
struct DoublyLinedList<T> {
    arena: Vec<Node<T>>,
    head_tail: Option<(Pointer, Pointer)>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    key: T,
    front: Option<Pointer>,
    back: Option<Pointer>,
}

use std::fmt::Debug;
use std::fmt::Display;
impl<T> Display for DoublyLinedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut str: String = String::new();
        match &self.head_tail {
            None => {
                str.push_str("empty");
            }
            Some((head, tail)) => {
                str.push_str(&format!("head: {}, tail:{}\n", head.0, tail.0));
            }
        }
        for (index, node) in self.arena.iter().enumerate() {
            str.push_str(&format!(
                "  {} key:{:?}, fr:{:?}, bk:{:?}\n",
                index, node.key, node.front, node.back
            ));
        }
        write!(f, "{}", str)
    }
}

impl<T> DoublyLinedList<T> {
    fn new() -> Self {
        Self {
            arena: vec![],
            head_tail: None,
        }
    }
    fn insert_head(&mut self, key: T) {
        let new_node;
        let new_head_pointer;
        let new_tail_pointer;
        match self.head_tail.take() {
            // empty case
            None => {
                new_node = Node {
                    key,
                    front: None,
                    back: None,
                };
                new_head_pointer = Pointer(self.arena.len());
                new_tail_pointer = Pointer(self.arena.len());
            }
            Some((old_head_pointer, old_tail_pointer)) => {
                new_node = Node {
                    key,
                    front: Some(old_head_pointer),
                    back: None,
                };
                new_head_pointer = Pointer(self.arena.len());
                new_tail_pointer = old_tail_pointer;

                self.set_back_pointer(old_head_pointer, Some(new_head_pointer));
            }
        }
        self.arena.push(new_node);
        self.head_tail = Some((new_head_pointer, new_tail_pointer));
    }
    fn insert_tail(&mut self, key: T) {
        let new_node;
        let new_head_pointer;
        let new_tail_pointer;
        match self.head_tail.take() {
            // empty case
            None => {
                new_node = Node {
                    key,
                    front: None,
                    back: None,
                };
                new_head_pointer = Pointer(0);
                new_tail_pointer = Pointer(0);
            }
            Some((old_head_pointer, old_tail_pointer)) => {
                new_node = Node {
                    key,
                    front: None,
                    back: Some(old_tail_pointer),
                };
                new_head_pointer = old_head_pointer;
                new_tail_pointer = Pointer(self.arena.len());

                self.set_front_pointer(old_tail_pointer, Some(new_tail_pointer));
            }
        }
        self.arena.push(new_node);
        self.head_tail = Some((new_head_pointer, new_tail_pointer));
    }
    fn set_front_pointer(&mut self, pointer: Pointer, front_pointer: Option<Pointer>) {
        self.get_mut_by_pointer(pointer)
            .map(|node| node.front = front_pointer);
    }
    fn set_back_pointer(&mut self, pointer: Pointer, back_pointer: Option<Pointer>) {
        self.get_mut_by_pointer(pointer)
            .map(|node| node.back = back_pointer);
    }
    fn delete_by_pointer(
        &mut self,
        pointer: Pointer,
    ) -> Option<(Option<Pointer>, Option<Pointer>)> {
        if pointer.0 < self.arena.len() {
            let del_node = &mut self.arena[pointer.0];
            let front = del_node.front.take();
            let back = del_node.back.take();
            match (front, back) {
                // middle case
                (Some(front_pointer), Some(back_pointer)) => {
                    self.set_back_pointer(front_pointer, Some(back_pointer));
                    self.set_front_pointer(back_pointer, Some(front_pointer));
                    Some((Some(front_pointer), Some(back_pointer)))
                }
                // head case
                (Some(front_pointer), None) => {
                    let (_head_pointer, tail_pointer) = self.head_tail.take().unwrap();
                    self.head_tail.replace((front_pointer, tail_pointer));
                    self.set_back_pointer(front_pointer, None);
                    Some((Some(front_pointer), None))
                }
                // tail case
                (None, Some(back_pointer)) => {
                    let (head_pointer, _tail_pointer) = self.head_tail.take().unwrap();
                    self.head_tail.replace((head_pointer, back_pointer));
                    self.set_front_pointer(back_pointer, None);
                    Some((None, Some(back_pointer)))
                }
                // head and tail case
                _ => {
                    self.head_tail = None;
                    Some((None, None))
                }
            }
        } else {
            None
        }
    }
    fn get_by_pointer(&self, pointer: Pointer) -> Option<&Node<T>> {
        self.arena.get(pointer.0)
    }
    fn get_mut_by_pointer(&mut self, pointer: Pointer) -> Option<&mut Node<T>> {
        self.arena.get_mut(pointer.0)
    }
    fn delete_first_key(&mut self, key: T)
    where
        T: PartialEq,
    {
        let mut pointer: Option<Pointer> = self.head_tail.as_ref().map(|(head, _)| head).cloned();
        while let Some(exist_pointer) = pointer {
            let node = self.get_by_pointer(exist_pointer).unwrap();
            if node.key == key {
                pointer = self
                    .delete_by_pointer(exist_pointer)
                    .and_then(|(front, _back)| front);
                return;
            } else {
                pointer = node.front;
            }
        }
    }
    fn delete_by_key(&mut self, key: T)
    where
        T: PartialEq,
    {
        let mut pointer: Option<Pointer> = self.head_tail.as_ref().map(|(head, _)| head).cloned();
        while let Some(exist_pointer) = pointer {
            let node = self.get_by_pointer(exist_pointer).unwrap();
            if node.key == key {
                pointer = self
                    .delete_by_pointer(exist_pointer)
                    .and_then(|(front, _back)| front);
            } else {
                pointer = node.front;
            }
        }
    }
    fn delete_first(&mut self) {
        if let Some((head_pointer, _)) = &self.head_tail {
            self.delete_by_pointer(*head_pointer);
        }
    }
    fn delete_last(&mut self) {
        if let Some((_, tail_pointer)) = &self.head_tail {
            self.delete_by_pointer(*tail_pointer);
        }
    }
}

struct Iter<'a, T> {
    list: &'a DoublyLinedList<T>,
    pointer: Option<Pointer>,
}

impl<T> DoublyLinedList<T> {
    fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: &self,
            pointer: self.head_tail.map(|(head, _)| head),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let opt_elm = self.list.get_by_pointer(self.pointer?);
        self.pointer = opt_elm.and_then(|exists| exists.front);
        opt_elm.map(|elem| &elem.key)
    }
}

#[derive(Debug, Clone)]
enum OrderDoubleyLinkedList {
    Insert(usize),
    Delete(usize),
    DeleteFirst,
    DeleteLast,
}

use std::convert::TryFrom;
impl TryFrom<&str> for OrderDoubleyLinkedList {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        let mut splitted = value.split_whitespace();
        match splitted.next().unwrap() {
            "insert" => Ok(OrderDoubleyLinkedList::Insert(
                splitted
                    .next()
                    .and_then(|str| str.parse::<usize>().ok())
                    .ok_or(())?,
            )),
            "delete" => Ok(OrderDoubleyLinkedList::Delete(
                splitted
                    .next()
                    .and_then(|str| str.parse::<usize>().ok())
                    .ok_or(())?,
            )),
            "deleteFirst" => Ok(OrderDoubleyLinkedList::DeleteFirst),
            "deleteLast" => Ok(OrderDoubleyLinkedList::DeleteLast),
            _ => Err(()),
        }
    }
}

fn main() {
    let orders: Vec<OrderDoubleyLinkedList> = input();
    let mut linked_list: DoublyLinedList<usize> = DoublyLinedList::new();
    for order in orders {
        match order {
            OrderDoubleyLinkedList::Insert(u) => linked_list.insert_head(u),
            OrderDoubleyLinkedList::Delete(u) => linked_list.delete_first_key(u),
            OrderDoubleyLinkedList::DeleteFirst => linked_list.delete_first(),
            OrderDoubleyLinkedList::DeleteLast => linked_list.delete_last(),
        }
    }
    for (i, v) in linked_list.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", v);
    }
    println!()
}

fn input() -> Vec<OrderDoubleyLinkedList> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    let n = {
        string.clear();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<usize>().unwrap()
    };
    (0..n)
        .map(|_| {
            string.clear();
            stdin.read_line(&mut string).unwrap();
            OrderDoubleyLinkedList::try_from(string.as_str()).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_test() {
        let mut linked_list: DoublyLinedList<usize> = DoublyLinedList::new();
        // linked_list.insert_head(4);
        // linked_list.insert_head(3);
        // linked_list.insert_head(2);
        // linked_list.insert_head(1);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![1, 2, 3, 4]);

        // linked_list.delete_by_pointer(Pointer(0));
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![1, 2, 3]);

        // linked_list.delete_by_key(1);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![2, 3]);

        // linked_list.delete_first();
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![3]);

        // linked_list.insert_head(2);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![2, 3]);

        // linked_list.insert_head(1);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![1, 2, 3]);

        // linked_list.insert_tail(4);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![1, 2, 3, 4]);

        // linked_list.delete_by_key(4);
        // let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(collect, vec![1, 2, 3]);

        linked_list = DoublyLinedList::new();
        linked_list.insert_head(7);
        linked_list.insert_head(3);
        linked_list.delete_first();
        linked_list.delete_last();
        eprintln!("{}", linked_list);
        let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(collect, vec![]);
        linked_list.insert_head(1);
        eprintln!("{}", linked_list);
        let collect = linked_list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(collect, vec![1]);

    }
}

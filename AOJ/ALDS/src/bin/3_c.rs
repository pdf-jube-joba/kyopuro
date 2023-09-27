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

impl<T> DoublyLinedList<T> {
    fn new() -> Self {
        Self { arena: vec![], head_tail: None }
    }
    fn insert(&mut self, key: T) {
        match self.head_tail.take() {
            None => {
                self.arena = vec![
                    Node {
                        key,
                        front: None,
                        back: None,
                    }
                ];
                self.head_tail = Some((Pointer(0), Pointer(0)));
            }
            Some((head_pointer, tail_pointer)) => {
                let node = Node {
                    key,
                    front: None,
                    back: Some(head_pointer),
                };
                self.arena.push(node);
                self.head_tail = Some((
                    Pointer(self.arena.len() - 1),
                    tail_pointer)
                );
                let head_node: &mut Node<T> = self.get_mut_by_pointer(head_pointer).unwrap();
                head_node.front = Some(head_pointer);
            }
        }
    } 
    fn delete_by_pointer(&mut self, pointer: Pointer) -> Option<(Option<Pointer>, Option<Pointer>)>  {
        if pointer.0 < self.arena.len() {
            let del_node = &mut self.arena[pointer.0];
            let front = del_node.front.take();
            let back = del_node.back.take();
            match (del_node.front.take(), del_node.back.take()) {
                // middle case
                (Some(front_pointer), Some(back_pointer)) => {
                    let front_node: &mut Node<_> = &mut self.arena[front_pointer.0];
                    front_node.back.replace(back_pointer);
                    let back_node: &mut Node<_> = &mut self.arena[back_pointer.0];
                    back_node.front.replace(front_pointer);
                    Some((
                        Some(front_pointer),
                        Some(back_pointer)
                    ))
                }
                // tail case
                (Some(front_pointer), None) => {
                    let (head_pointer, _) = self.head_tail.take().unwrap();
                    self.head_tail.replace(
                        (head_pointer, front_pointer)
                    );
                    Some((
                        Some(front_pointer),
                        None
                    ))
                }
                // head case
                (None, Some(back_pointer)) => {
                    let (_, tail_pointer) = self.head_tail.take().unwrap();
                    self.head_tail.replace(
                        (back_pointer, tail_pointer)
                    );
                    Some((
                        None,
                        Some(back_pointer)
                    ))
                }
                // head and tail case
                _ => {
                    self.head_tail = None;
                    Some((
                        None,
                        None,
                    ))
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
    fn delete(&mut self, key: T) where T: PartialEq {
        let mut pointer: Option<Pointer> = self.head_tail.as_ref().map(|(head, _)| head).cloned();
        while let Some(exist_pointer) = pointer {
            let node = self.get_by_pointer(exist_pointer).unwrap();
            if node.key == key {
                pointer = self.delete_by_pointer(exist_pointer).and_then(|(_, tail)| tail);
            } else {
                pointer = node.back;
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
            pointer: self.head_tail.map(|(head, _)| head)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let to_item = self.list.get_by_pointer(self.pointer?);
        self.pointer = to_item.and_then(|exists| exists.back);
        to_item.map(|elem| &elem.key)
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
            "insert" =>
                Ok(OrderDoubleyLinkedList::Insert(
                    splitted.next().and_then(|str|{
                        str.parse::<usize>().ok()
                    }).ok_or(())?
                )),
            "delete" =>
                Ok(OrderDoubleyLinkedList::Delete(
                    splitted.next().and_then(|str|{
                        str.parse::<usize>().ok()
                    }).ok_or(())?
                )),
            "deleteFirst" =>
                Ok(OrderDoubleyLinkedList::DeleteFirst),
            "deleteLast" =>
                Ok(OrderDoubleyLinkedList::DeleteLast),
            _ => Err(()),
        }
    }
}

fn main() {
    let orders: Vec<OrderDoubleyLinkedList> = input();
    let mut linked_list: DoublyLinedList<usize> = DoublyLinedList::new();
    for order in orders {
        match order {
            OrderDoubleyLinkedList::Insert(u) => {
                linked_list.insert(u)
            }
            OrderDoubleyLinkedList::Delete(u) => {
                linked_list.delete(u)
            }
            OrderDoubleyLinkedList::DeleteFirst => {
                linked_list.delete_first()
            }
            OrderDoubleyLinkedList::DeleteLast => {
                linked_list.delete_last()
            }
        }
        eprintln!("{:?}", linked_list);
    }
    let mut pointer = linked_list.head_tail.map(|(head, _)| head);
    while let Some(exist_pointer) = pointer {
        let node = linked_list.get_by_pointer(exist_pointer).unwrap();
        println!("{} ", node.key);
        pointer = node.back;
    }
}

fn input() -> Vec<OrderDoubleyLinkedList> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    let n = {
        string.clear();
        stdin.read_line(&mut string).unwrap();
        string.trim().parse::<usize>().unwrap()
    };
    (0..n).map(|_|{
        string.clear();
        stdin.read_line(&mut string).unwrap();
        OrderDoubleyLinkedList::try_from(string.as_str()).unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_test() {
        let mut linked_list: DoublyLinedList<usize> = DoublyLinedList::new();
        eprintln!("{:?}", linked_list);
        linked_list.insert(1);
        eprintln!("{:?}", linked_list);
        linked_list.insert(2);
        eprintln!("{:?}", linked_list);
        linked_list.insert(3);
        eprintln!("{:?}", linked_list);
        linked_list.insert(4);
        eprintln!("{:?}", linked_list);

        let collect = linked_list.iter().cloned().collect::<Vec<_>>();

        assert_eq!(collect, vec![4,3,2,1]);
        linked_list.delete(1);
        eprintln!("{:?}", linked_list.iter().cloned().collect::<Vec<_>>());

        linked_list.delete_first();
        eprintln!("{:?}", linked_list);

    }
}

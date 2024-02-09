use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, VecDeque},
};

fn main() {
    proconio::input! {
        n: usize, m: usize,
        tws: [(usize, usize, usize); m],
    }
    let r = get_somen_sum(n, tws);
    for r in r {
        println!("{}", r);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Person(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Time(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Much(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Event {
    Get(Much, Time),
    Come(Person),
}

fn get_somen_sum(n: usize, tws: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut get_somen_sum = vec![0; n];

    let mut queue: BinaryHeap<(Reverse<usize>, Event)> = BinaryHeap::new();

    // initialize queue
    for &(t, w, s) in &tws {
        queue.push((Reverse(t), Event::Get(Much(w), Time(s))));
    }

    let mut order: BTreeSet<Person> = BTreeSet::from_iter((0..n).map(Person));

    while let Some((Reverse(t), ev)) = queue.pop() {
        match ev {
            Event::Get(Much(w), Time(s)) => {
                let Some(i) = order.pop_first() else {
                    continue;
                };
                get_somen_sum[i.0] += w;
                queue.push((Reverse(t + s), Event::Come(i)));
            }
            Event::Come(i) => {
                order.insert(i);
            }
        }
    }

    get_somen_sum
}

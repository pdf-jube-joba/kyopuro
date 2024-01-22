use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    proconio::input! {
        n: usize,
        td: [(usize, usize); n],
    }
    println!("{}", max_print(td));
}

fn max_print(mut td: Vec<(usize, usize)>) -> usize {
    let n = td.len();
    td.sort();

    let mut print: Vec<usize> = vec![];
    let mut time = 0;

    // = { i | t_i <= time <= t_i + d_i && i is not printed } ordered by (t_i + d_i)
    let mut priority_queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    loop {
        // print on top of pri.que. in time
        if let Some((p, i)) = priority_queue.pop() {
            print.push(i);
        }

        // forward time
        if priority_queue.is_empty() {
            // if empty, skip time to the min { t_i | time < t_i }
            let ind = td.partition_point(|&(t_i, d_i)| t_i <= time);
            if ind == n {
                break;
            }
            time = td[ind].0
        } else {
            time += 1;
        };
        // time before change < after change
        // => loop should stop

        // insert { i | t_i == time after change } to pri.que (it's not printed)
        let mut ind = td.partition_point(|&(t_i, d_i)| t_i < time); // t_i < time if i in 0..ind
        while ind < n && td[ind].0 == time {
            priority_queue.push((Reverse(td[ind].0 + td[ind].1), ind));
            ind += 1;
        }

        // dispose { i | t_i + d_i < time after change } in pri.que (we cannot print on it)
        while priority_queue
            .peek()
            .filter(|&(Reverse(ti_di), ind)| *ti_di < time)
            .is_some()
        {
            priority_queue.pop();
        }

        // now pri.que = { i | t_i <= time after change <= t_i + d_i && i is not printed }
    }
    print.len()
}

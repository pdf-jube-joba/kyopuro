fn insert(v: &mut Vec<(usize, usize)>, val: (usize, usize)) {
    let res = (0..v.len()).find(|i| val.1 >= v[*i].1);
    match res {
        Some(i) => {
            v.insert(i, val);
        }
        _ => {
            v.push(val);
        }
    }
}

fn dijkstra(graph: Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let n = graph.len();
    debug_assert!(n > 0);
    let mut v: Vec<Option<usize>> = vec![None; n];
    v[0] = Some(0);
    let mut nexts: Vec<(usize, usize)> = vec![];

    // push edge from vertex 0 
    for (ind, cost) in &graph[0] {
        nexts.push((*ind, *cost));
    }

    nexts.sort_by(|(_, cost1), (_, cost2)| cost2.cmp(&cost1));
    
    while let Some((ind, this_cost)) = nexts.pop() {
        if v[ind].is_none() {
            v[ind] = Some(this_cost);
            for (ind_neibor, cost) in &graph[ind] {
                if v[*ind_neibor].is_none() {
                    insert(&mut nexts, (*ind_neibor, this_cost + cost));
                }
            }
        }
    }

    v.into_iter().map(|v| v.unwrap()).collect()
}

fn main() {
    let list = input();
    let costs = dijkstra(list);
}

fn input() -> Vec<Vec<(usize, usize)>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    let mut v = (0..n).map(|_|{
        buf.clear();
        let v = buf.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let ind = v[0];
        let deg = v[1];
        (ind, (0..deg).map(|i| (v[2*i], v[2*i+1])).collect())
    }).collect::<Vec<(usize, Vec<(usize, usize)>)>>();
    v.sort_by(|(i1, _), (i2, _)| i1.cmp(&i2));
    v.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_test() {
        let mut v = vec![];
        insert(&mut v, (0,0));
        assert_eq!(v, vec![(0,0)]);

        insert(&mut v, (0,5));
        assert_eq!(v, vec![(0,5), (0,0)]);

        insert(&mut v, (2,3));
        assert_eq!(v, vec![(0,5), (2,3), (0,0)]);

        insert(&mut v, (1,4));
        assert_eq!(v, vec![(0,5), (1,4), (2,3), (0,0)]);

        insert(&mut v, (0,6));
        assert_eq!(v, vec![(0,6), (0,5), (1,4), (2,3), (0,0)]);
    }
    #[test]
    fn dijk_test() {
        let graph = vec![
            vec![],
        ];
        assert_eq!(dijkstra(graph), vec![0]);

        let graph = vec![
            vec![(1, 0)],
            vec![],
        ];
        assert_eq!(dijkstra(graph), vec![0, 0]);

        let graph = vec![
            vec![(1, 2)],
            vec![],
        ];
        assert_eq!(dijkstra(graph), vec![0, 2]);

        let graph = vec![
            vec![(1, 1), (2, 5)],
            vec![(2, 2)],
            vec![],
        ];
        assert_eq!(dijkstra(graph), vec![0, 1, 3]);

        let graph = vec![
            vec![(1, 1), (2, 5)],
            vec![(2, 2)],
            vec![(0,1), (1, 1)],
        ];
        assert_eq!(dijkstra(graph), vec![0, 1, 3]);
    }
}
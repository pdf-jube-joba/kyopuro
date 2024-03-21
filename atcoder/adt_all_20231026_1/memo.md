# h
解説面白かった確かに。

```rust
fn bishop2(a: (usize, usize), b: (usize, usize), s: Vec<Vec<bool>>) -> Option<usize> {
    let n = s.len();
    if (a.0 + a.1) % 2 != (b.0 + b.1) % 2 {
        return None;
    }
    let moveto = |f: (usize, usize), d: (isize, isize)| -> Option<(usize, usize)> {
        if 0 <= f.0 as isize + d.0
            && f.0 as isize + d.0 < n as isize
            && 0 <= f.1 as isize + d.1
            && f.1 as isize + d.1 < n as isize
        {
            Some(((f.0 as isize + d.0) as usize, (f.1 as isize + d.1) as usize))
        } else {
            None
        }
    };
    let mut visited: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((a, 0));
    while let Some((next, step)) = queue.pop_front() {
        if visited[next.0][next.1].is_some() {
            continue;
        }
        visited[next.0][next.1] = Some(step);
        for dir in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            for d in 0..n {
                if let Some(t) = moveto(next, (d as isize * dir.0, d as isize * dir.1)) {
                    if s[t.0][t.1] {
                        break;
                    }
                    queue.push_back((t, step + 1));
                } else {
                    break;
                }
            }
        }
    }
    visited[b.0][b.1]
}
```
これは O(n^3) 解法ではあるがちょうど Ω(n^3) か？
（各マスごとに n に比例するぐらいのマスを queue に入れてるので）

ダイクストラ法で近くの辺をたどる際に「dist[w] >= dist[v] ならなにかする」を毎回忘れる。
正当性がわかってないかも。

あと、他の提出みてたら、01bfsでない回答なのに早い奴があった。
https://atcoder.jp/contests/abc246/submissions/48257153
なんで？

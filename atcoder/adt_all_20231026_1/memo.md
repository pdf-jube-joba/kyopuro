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
枝かりをするらしい
https://drken1215.hatenablog.com/entry/2022/04/03/192300

# i
$a[0 \ldots n]$ にたいして各 $k in 0 \ldots= n$ ごとに $(\sum_{i \in 0 \ldots k, j \in 0 \ldots k} \max(a[i], a[j])) / k^2$ を求める。
各 $k$ ごとに $\sum \cdots$ の部分を求めれることを考えると、 $k$ から $k+1$ ができる。
$S[i \in 0 \ldots= n] = \sum_{i \in 0 \ldots k, j \in 0 \ldots k} \max(a[i], a[j])$ とする。
- $S[0] = 0$
- $k in 0\ldots n$, $S[k+1] = S[k] + 2 * \sum_{i \in 0 \ldots k} \max(a[i], a[k]) + a[k]$
と書けて、求めるのは $k in 0..=n$ ごとに $S[k] / k^2$ となる。
次に $k in 0\ldots n$, $s[k in 0 \ldots n] = \sum_{i in 0 \ldots k} \max(a[i], a[k])$ を求めたい。
適当に分ければ、 $s[k in 0 \ldots n] = (\sum_{i in 0 \ldots k, a[i] \leq a[k]}) * a[k] + \sum_{i in 0 \ldots k, a[i] > a[k]} a[i]$ となる。

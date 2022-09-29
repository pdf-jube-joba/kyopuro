# 問題
前提
- $a_{i}$: $1 \leq i \leq n$ に対して整数
- $k$: 整数
---
求めるもの
- $\text{min}\{\sum_{i} b_i \mid \text{\(b_i \geq 0\) such that \(\#\{j \mid \forall i \leq j, a_i + b_i \leq a_j + b_j\} \geq k\)}\}$
---
制約
- $1 \leq k \leq n \leq 15$
- $1 \leq a_i \leq 10^9$

# 回答 1
## 方針
$\{1 ..= k\}$ の部分集合 $J$ に対して、 $J \subset \{j \mid \forall i \leq j, a_i + b_i \leq a_j + b_j\}$ を満たす $b_i$ を考えて、 $\sum b_i$ を最小化することが非常に容易にできる。
これは少なくとも $j \in J$ のところでそれまでに出た最大の $n = a_i + b_i$ を覚えておけば次に $b_j = \text{max}(n - a_i, 0)$ とすることで $b_i$ が与えられる。
これを用いて全ての部分集合 $J$ であって $\# J \leq k$ を満たすものに対して最小値を計算して、全体で最小値を求めればよい。
## 実装
```Rust
fn minimize(a: &[usize], k: usize) -> usize {
    let n = a.len();
    (0..(1 << n)).map(|i|{convert(n, i)}).filter_map(|bits|{
        if !(bits.iter().filter(|b|**b).count() < k) {
            let mut b: Vec<usize> = vec![0; n];
            let mut now = 0;
            (0..n).for_each(|i|{
                if bits[i] && now > a[i] { b[i] = now - a[i] + 1; }
                now = std::cmp::max(now, a[i] + b[i])
            });
            let sum = b.iter().sum();
            Some(sum)
        } else {None}
    }).min().unwrap()
}
```
## メモ
やっぱし `filter_map` よみにくいかも。
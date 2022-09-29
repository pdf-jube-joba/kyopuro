# 問題
前提
- $r$, $c$,: 整数
- $a_{i,j}$: $r \times c$ の真偽値の行列
---
求めるもの
- 各 $I = \{i_1, \ldots, i_{k_r}\} \subset \{1 ..= r\}$, $J = \{j_1, \ldots, j_{k_c}\} \subset \{1..=c\}$ に対して $(a^{I,J})_{i,j}$ を $[i \in I] \oplus [j \in J] \oplus a_{i,j}$ で定めたとき、 $\text{max}\{ \text{number of \((i,j)\) such that \((a^{I,J})_{i,j}\) is \(\top\)} \mid I,J\}$
---
制約
- $1 \leq r \leq 10$
- $1 \leq c \leq 10000$

# 回答 1
## 方針
今 $I$ を固定したとする。
$J$ であって $[\text{number of \((i,j)\) such that \((a^{I,J})_{i,j} = 1\)}]$ が最大になるようなものを探したい。
このとき $(a^{I,J})_{i,j}$ の $j$ 列目は $j \in J$ かどうかがわかれば計算でき、各 $j$ に対して $[\text{number of \(i\) such that \([i \in I] \oplus [j \in J] \oplus a_{i,j}\) is \(\top\)}]$ が最大になるように $j \in J$ かどうかを設定すればよい。
## 実装
```Rust
fn maximize(a: &[Vec<bool>]) -> usize {
    let r = a.len();
    let c = a[0].len();
    (0..(1 << r))
    .map(|i|(convert(r, i)))
    .map(|bits|{
        let a: Vec<Vec<bool>> = a.into_iter().enumerate().map(|(i, vec)|{
            vec.into_iter().map(|j|{bits[i] ^ j}).collect()
        }).collect();
        (0..c).map(|i|{
            let count = (0..r).filter(|j|{a[*j][i]}).count();
            std::cmp::max(count, r - count)
        }).sum()
    }).max().unwrap()
}
```
各部分集合に対して、 $J = \empty$ の場合の $a^{I,J}$ を計算し、 `a` に再代入（シャドーイング）している。
その後各 $j$ に対して $j \in J$ と $j \not \in J$ のどちらがよいかを `std::cmp::max(count, r-count)` で判定している。

# 回答 2
## 方針
同様
## 実装
```Rust
fn maximize(a: &[Vec<bool>]) -> usize {
    let r = a.len();
    let c = a[0].len(); // a[i].len() == a[j].len()
    (0..(1 << r))
    .map(|i|(convert(r, i)))
    .map(|bits|{
        (0..c).map(|j|{
            let count = (0..r)
                .filter(|&i| a[i][j] ^ bits[i])
                .count();
            std::cmp::max(count, r - count)
        }).sum()
    }).max().unwrap()
}
```
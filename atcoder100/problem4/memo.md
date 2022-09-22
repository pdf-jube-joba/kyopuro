# 問題
前提
- $n$: 整数
- $m$: 整数
- $a_{i,j}$ for $1 \leq i \leq n, 1 \leq j \leq m$: 整数
---
求めるもの
- $\text{max}\{ \sum_{1 \leq j \leq m} \text{max} (a_{i_1, j}, a_{i_2, j}) \mid i_1 \neq i_2 \in \{1..n\} \}$
---
制約
- $1 \leq n \leq 100$
- $2 \leq m \leq 100$
- $0 \leq a_{i,j} \leq 100\_000\_000$

# 方針
$i_1, i_2$ について $\sum_{1 \leq j \leq m} \cdots$ を計算して max を見ればよい

# 実装のメモ
これは一番先に for 文を回したものを書いた。
`a: &[Vec<usize>]` を受け取って
```Rust
let n = a.len();
let m = a[0].len();
```
として $n$ や $m$ を復元した。
気を付ける点として
- $a_{i,j}$ の添え字はともに $1$ から始まり、コード上では $0$ 始まりの collection によって表現されているが、添え字に関して特に調整してないのは input の段階で（自動的に）調整されているから。
- $a$ を `&[Vec<usize>]` で受け取り `n` や `m` を `a` から計算しているが、 `m` については `a[i].len() == a[j].len()` が成り立つことと $1 \leq n$ であることから `a[0]` なるアクセスが必ず成功して正しい。
## 1
コミットしたものを少し変更している。
```Rust
let mut max = 0;
for i in 0..m {
    for j in i..m {
        let mut sum = 0;
        for k in 0..n {
            sum += std::cmp::max(a[k][i], a[k][j]);
        }
        max = std::cmp::max(max, sum);
    }
}
max
```
- $0 \leq i \leq j < m, 0 \leq k < n$ のため `a[k][i]`,`a[k][j]` のアクセスも成功する。
- max や sum はその名前の通りの条件を満たして for 文を回る。

あとはほとんど素直なコードなので正しさの検証は（長くはなるけど） straightforward に行きそう。

## 2
もうちょっと（自分用に）整理されたコードを書きたい。
```Rust
(0..m)
.flat_map(|i|{
    (i..m).map(move |j| (i,j))
})
.map(|(i,j)|{
    (0..n).map(|k| std::cmp::max(a[k][i], a[k][j])).sum()
})
.max().unwrap()
```
- `unwrap()` に成功するのは `flat_map` の時点で $2 \leq m$ より iterator が空でないからよい。

## 比較
- 前のコードでは `sum` や `max` など（変数名から役割は察せられるが）変数の持っていた条件を検証するのがだるい。
- こちらのコードは `sum` や `max` などの関数を用いているので、検証がより簡単。
- その分 unwrap などを書く必要がある。
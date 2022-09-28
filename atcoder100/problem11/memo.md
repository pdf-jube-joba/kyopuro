# 問題
前提
- $n$, $m$: 整数
- $s_{i} = (s_{i,1} \ldots s_{i,k_{i}})$: 各 $1 \leq i \leq m$ に対して、相異なる $\{1 ..= n\}$ の元
- $p_i$: $\{0,1\}$ の元
---
求めるもの
- $\{1 ..=n\}$ の部分集合であって、全ての $1 \leq i \leq m$ に対して $\# \{s_{i,1}, \ldots, s_{i, k_i}\} \cap \{1 ..= n\} \equiv p_i (\text{mod \(2\)})$ が成り立っているものがいくつあるか。
---
制約
- $1 \leq n,m \leq 10$

# 回答 1
## 方針
$\{1..=n\}$ の部分集合をすべて列挙し、それが条件を満たすかどうかを判定する。
## 実装
`convert` については problem10 にあるものを使う。
各部分集合に対して、
`switch_pushed` はその $i$ 番目が $\#\{s_{i,1},\ldots,s_{i,k_i}\} \cap \{1..=n\}$ となるように計算している。
ただし、 index の問題により、 $switch-1$ のようなものがあらわれている。
この `switch_pushed` により、 $i$: $\{1..=m\}$ の全てが $\text{押された個数} \equiv p[i] (\text{mod \(2\)})$ になるかどうかを考えて、これが `true` になるものを数え上げている。

```Rust
fn count(n: usize, s: &[Vec<usize>], p: &[usize]) -> usize {
    let m = s.len();
    (0..(1 << n))
    .map(|i|convert(n, i))
    .filter(|bits|{
        let switch_pushed: Vec<usize> = s.iter().map(|switches|{
            switches.iter().filter(|switch|{bits[**switch-1]}).count()
        }).collect();
        (0..m).all(|i|{switch_pushed[i] % 2 == p[i]})
    }).count()
}
```

## その他メモ
書いた順番としては割と上から書いたのだが、説明する順番や計算する順番と、コードに現れる順番が異なるため、やや読みにくいかも。
例えば、順序としては `filter` を行うのは一番最後なのだがコードの上では早めに表れている。
これは、「最終的に、○○が `true` になるものを数え上げる問題になるから、 `filter`,`count`を書いてしまう」として説明したほうがいいかもしれない。

## 計算量
`switch_pushed` を作るために、行われる計算は `bits` によらずに一定であり、全ての $s$ を走査するので、 $\lVert s \rVert$ になる。
一方で $i$: $\{1..=m\}$ の全てが条件を満たすかどうかの計算は `bits` によって異なり、最悪 $m$ となる。
ので $O(2^n * (\lvert s \rvert * \lVert s \rVert))$ になる。

# 回答 2
## 方針
一度 `switch_push` を計算するときに　collect をしていたが、その必要はない。
## 実装
```Rust
(0..(1 << n))
.map(|i|convert(n, i))
.filter(|bits|{
    (0..m).all(|i|{
        s[i].iter().filter(|switch|bits[**switch-1]).count() % 2 == p[i]
    })
}).count()
```
（少し無理やりだけど、） $\{1..=m\}$ の全ての $i$ について $s[i]$ の元であってその `bits` が立っているものの数が、 $2$ で割った余りが $p[i]$ になっているか、と素直に読める気がする。
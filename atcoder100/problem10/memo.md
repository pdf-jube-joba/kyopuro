# 問題
前提
- $a$: $n$ の数列
- $m_{i}$: $q$ 個の整数
---
求められるもの
- 各 $m_i$ が $a$ の要素を足し合わせて作ることができるかどうか
---
制約
- $1 \leq n \leq 20$
- $1 \leq q \leq 200$
- $1 \leq \text{\(a\) の要素} \leq 2000$
- $1 \leq m_i \leq 2000$

# 回答 1
## 方針
$a$ の部分集合をすべて列挙し、その和が$m_i$になっているかどうかをすべて判定する。
すなわち、ビット全探索を行う。
$n$, $q$, $m_i$, $a$ の要素がすべて $0$ より大きいので、型には `usize` を用いる。

## 実装
ビット全探索を行うために、 整数 $a$ の2進数表記を行った際の下から $n$ ビットを計算するものを書く。
```Rust
fn convert(n: usize, a: usize) -> Vec<bool> {
    (0..n).map(|i|{
        a & (1 << i) != 0
    }).collect()
}
```
次に $a$: 配列と $m$: 整数を受け取って作ることができるかどうかを判定する関数を書く。
```Rust
fn make(a: &[usize], m: usize) -> bool {
    let n = a.len();
    for bits in (0..(1 << n)).map(|i|{convert(n, i)}) {
        let sum: usize = (0..n).map(|i|{
            if bits[i] {a[i]} else {0}
        }).sum();
        if sum == m {return true;}
    }
    return false
}
```
for 文で回す `bits` は `Vec<bool>` で受け取っている。
エラーになる部分はなさそう。
正しさの検証についても同じく。
## 計算量
すべての部分集合をまわるので $O(2^n)$ になる。

# 回答 2
## 方針
先ほどと同じ。
`make` をもう少しきれいに書きたい。
部分集合のうち和が $m$ になるものが"一つでも"あればよいのでそのような関数を用いる。
## 実装
```Rust
let n = a.len();
(0.. (1 << n))
.map(|i| convert(n,i))
.any(|bits|{
    (0..n)
    .filter(|&i| bits[i])
    .map(|i|a[i])
    .sum::<usize>() == m
})
```
`bits` のうち、 $a[i]$ が `true` になる $a$ の要素をすべて足し合わせたものが $m$ になるものが、一つでもあればよい。
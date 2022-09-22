# 問題
前提
- $n$: 整数
---
求めるもの
- $\#\{1 \leq i \leq n \mid \text{\(i\) は奇数}, \text{\(i\) の約数は \(8\) 個}\}$
---
制約
- $1 \leq n \leq 200$

# 方針
すべての $1 \leq i \leq n$ について条件を満たすかどうかを調べてそのような $i$ を数える。

# 実装のメモ
## 1
数え上げればよいので次のように書ける（はず）。
```Rust
let mut count = 0;
for i in 1..=n {
    if i % 2 == 1 && {
        let mut num = 0;
        for j in 1..=i {
            if i % j == 0 {num += 1;}
        }
        num == 8
    } {count += 1;}
}
return count;
```
`count += 1` が実行されるのが `i % 2 == 1 && num == 8` のときなのでよい。

## 2
次のようにも書ける。
```Rust
(1..=n).filter(|i| {
    i % 2 == 1 && (1..=*i).filter(|j|{i % j == 0}).count() == 8
}).count()
```

# 計算量？
どちらの実装を使うにせよ、$i$ をすべて調べ各 $i$ について約数を数えあげるので、だいたい $O(n^2)$ とかになりそう。
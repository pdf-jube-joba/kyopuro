# 問題
前提
- $n$, $m$: $1$ 以上の整数
- $(x_i, y_i)_{1 \leq i \leq m}$: それぞれ $\{1 ..= n\}$ の整数で $x_i < y_i$
---
求めるもの
- $\{1 ..= n\}$ の部分集合 $S$ であってどの $x,y \in S$ で $x<y$ を満たすものについても $(x,y) \in \{(x_i, y_i)\}$ となる、そんな $S$ について $\text{max} \{\# S \mid \text{そんな \(S\)}\}$ を求めよ
---
制約
- $1 \leq n \leq 12$
- $(x_i, y_i) \neq (x_j, y_j)$ if $i \neq j$

# 回答 1
## 方針
部分集合 $S$ をすべて列挙し、条件を満たすかどうかによりふるいにかけ、その集合の大きさの最大値を計算する。

## 実装
convert については problem10 に書いたものを用いる。

```Rust
(0..(1 << n)).map(|i| convert(n, i)).filter_map(|bits|{
    let all: Vec<(usize, usize)> = (0..n).map(|i|{
        (i+1..n).map(move |x|{(i, x)})
    }).flatten().filter(|(i,j)|{bits[*i] && bits[*j]}).collect();
    let result = all.iter().all(|(i,j)|{
        rel.iter().any(|(x,y)|{*x - 1 == *i && *y - 1 == *j})
    });
    if result {Some(bits.iter().filter(|b|**b).count())} else {None}
}).max().unwrap()
```

エラーになる可能性がある `unwrap()` を用いているが、少なくとも空集合は条件を満たすから iterator は空ではない。

## 計算
部分集合の列挙なので $O(2^n * \alpha)$ になる。

# 回答 2
## 方針
いろいろと改善点があるように思えた。
- `flatten` よりも `flat_map` で書いたほうがいい。
- `collect` せずにそのまま `all` につなげる。
- `rel.iter.any` 部はようするに `contains` なのだが、 index のずれによりこのように書かざるを得ない。 rel を index をずらして再定義しなおす。 $(x_i, y_i)$ の制限により $-1$ しなおしてもエラーにならない。

## 実装
```Rust
(0..(1 << n)).map(|i| convert(n, i))
.filter(|bits|{
    (0..n)
    .flat_map(|x|{
        (x+1..n).map(move |y|(x,y))
    }) 
    .filter(|&(i,j)| bits[i] && bits[j])
    .all(|xy| rel.contains(&xy))
})
.map(|bits| 
    bits.iter().filter(|b|**b).count()
)
.max().unwrap()
```

これよりも短く書くことはできると思うが、説明するのには向いてないかもと思った。
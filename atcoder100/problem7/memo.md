# 問題
前提
- $p$: $\mathbb{Z}_{\geq 0} \times \mathbb{Z}_{\geq 0}$ の配列
---
求めるもの
- $\text{max}\{\text{\(ABCD\) の面積} \mid \text{\(A,B,C,D\) は \(p\) の点で \(ABCD\) は正方形}\}$
---
- $1 \leq \lvert p \rvert \leq 3000$
- $0 \leq \text{\(p\) の座標} \leq 5000$

# 回答1
## 方針
正方形はある線分から復元することができる。
そのため、 $p$ の各二点 $a$, $b$ について点 $c$ $d$ であって $abcd$ が正方形になるものを算出し $c$, $d$ が $p$ に含まれるかどうかを判定しつつその面積を求めればよい。

## 実装
とりあえず上の方針に従って書いた。
二重の for 文を回し、
型については、 $a$ 自体は `&[(usize, usize)]` で受け取ってもよいが、座標の足し引きをする際にめんどくさいので、`i32` で受け取ってしまった。
座標に関する制限から `i32` で十分間に合う。
area の計算についても、面積は最大で $(5000 * \sqrt{2})^2 = 5 \times 10^7$ なので、 `i32` で計算して `usize` にしてしまってよい。
```Rust
fn output(a: &[(i32, i32)]) -> usize {
    let mut max = 0;
    for x1 in a {
        for x2 in a {
            let v: (i32, i32) = (x2.1 - x1.1, - (x2.0 - x1.0));
            let x3: (i32, i32) = (x1.0 + v.0, x1.1 + v.1);
            let x4: (i32, i32) = (x2.0 + v.0, x2.1 + v.1);
            let result =
                a.into_iter().any(|y|{y.0 == x3.0 && y.1 == x3.1}) &&
                a.into_iter().any(|y|{y.0 == x4.0 && y.1 == x4.1});
            let area = (x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2);
            if result {
                max = std::cmp::max(max, area);
            }
        }
    }
    max.try_into().unwrap()
}
```
- 計算量については実はよくないことがわかる。
- area が必ず正になることを含めて、必要のない変換等が発生している気がする。
- また、 max という変数名に意味を込めすぎている気がする。
- `a.into_iter()` は `a.iter()` でよいし、そもそも `a.contains(x3)` でやりたいことができる。

## 計算量
for 文の中身は $\lvert a \rvert^2$ 回実行される。
そのうち、 result を得るのに $a$ を線形に走査している。
そのため内部は最悪 $O(\lvert p \rvert)$ になるので全体は $O(\lvert p \rvert^3)$ となりよくない。

# 回答2
## 方針
解説においては、 `a` を辞書順に sort することで、走査を binary_search により行う方針を示していた。

## 実装
ついでに二重に for 文を回して max をえるのを変更する。
`p` を sort して所有する。
```Rust
let p = {
    let mut p = p.to_owned();
    p.sort();
    p
};
p.iter()
.flat_map(|i|{
    p.iter().map(move |j|{(i,j)})
})
.filter_map(|(&x1,&x2)|{
    let v: (i32, i32) = (x2.1 - x1.1, - (x2.0 - x1.0));
    let x3: (i32, i32) = (x1.0 + v.0, x1.1 + v.1);
    let x4: (i32, i32) = (x2.0 + v.0, x2.1 + v.1);
    if p.binary_search(&x3).is_ok() && p.binary_search(&x4).is_ok() {
        Some(((x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2)) as usize)
    } else {None}
})
.max()
.unwrap()
```

エラーについては
- `i32` を使っているのと値の範囲を考えると overflow はしない。
- `unwrap()` を使っているが、 `x1,x2` が同じ点の時が iterator に含まれるため、 result が必ず true になるので、filter_map 後の iterator は空でない。
- `a` は sort されているので `binary_search(&x3)` などは正しい値を返す。

停止は必ずする。

正しさについては方針のところで議論したので良い。（実装を間違ってなければ。）

## 計算量
binary search を用いたので for 文の中身が計算量が $O(\log \lvert p \rvert)$ になるから全体としては $O(\lvert p \rvert^2 \log \lvert p \rvert)$

# 回答3
## 方針
わざわざ vec を用いずに、適切な collection を用いることでより早く処理することができる。 HashSet を用いる。
## 実装
```Rust
let p: HashSet<(i32, i32)> = HashSet::from_iter(p.iter().cloned());
p.iter()
.flat_map(|i|{
    p.iter().map(move |j|{(i,j)})
})
.filter_map(|(&x1,&x2)|{
    let v: (i32, i32) = (x2.1 - x1.1, - (x2.0 - x1.0));
    let x3: (i32, i32) = (x1.0 + v.0, x1.1 + v.1);
    let x4: (i32, i32) = (x2.0 + v.0, x2.1 + v.1);
    if p.contains(&x3) && p.contains(&x4) {
        Some(((x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2)) as usize)
    } else {None}
})
.max()
.unwrap()
```
## 計算量
この計算量がどうなるかは、`filter_map`の内部の部分が定数で行えるので $O(\lvert p \rvert^2)$ になる。
# 問題
前提
- $n$: 整数
- $a_i\leq b_i$: 各（$i$ : $\{1..=n\}$）に対して $\{1..=10^9\}$
---
求めるもの
- $\text{min}\{\sum_{1 \leq i \leq n} p_i(s,t) \mid 1 \leq s,t \leq 10^9 \}$
ただし、
- $p_i(s,t) = \text{min of}$ 
    - (1) $\lvert s - a_i \rvert + \lvert a_i - b_i \rvert + \lvert b_i - t \rvert$
    - (2) $\lvert s - b_i \rvert + \lvert b_i - a_i \rvert + \lvert a_i - t \rvert$
---
制約
- $1 \leq n \leq 30$
注意点
- 買い物客がどう行動するのかがよくわからなかった。
「次のような経路」とは書かれているが、「次を満たす経路」とは書かれていないので、「まず入口からスタートする...」 で一意に定まる経路があるのかもしれないと考えた結果、その一意な経路は最適に行動したときの経路であり、その移動時間の合計が上記のように書けると考えた。
なので、問題の定式化が正しくない可能性がある。

実際の値としては同じになるが、候補としては次のようなものがあると思う
- $\text{min} \{\sum_{1 \leq i \leq n} p_i(s,t) \mid 1 \leq s,t \leq 10^9 \}$
- $\text{min} \{\sum_{1 \leq i \leq n} \text{min} \{\text{path} \mid \text{path は \(s\) に始まり \(t\) に終わり、 \(a_i,b_i\) を経由する}\} \mid 1 \leq s,t \leq 10^9 \}$
- $\text{min} \{\sum_{1 \leq i \leq n} \text{path} \mid 1 \leq s,t \leq 10^9, \text{path は \(s\) に始まり \(t\) に終わり、 \(a_i,b_i\) を経由する}\}$

実際に同じになることについては証明はほとんど作業なのでやめておく。

# 方針
## 1
実は計算により、
$\text{min}\{\sum_{1 \leq i \leq n} \lvert s - a_i \rvert + \lvert a_i - b_i \rvert + \lvert b_i - t \rvert \mid 1 \leq s \leq t \leq 10^9 \}$ でよいとわかる。
- 実際、$s \leq t$ の仮定の下で $(1) \leq (2)$ であることがわかり、 $t \leq s$ の仮定の下で $(2) \leq (1)$ でありかつ $s,t$ を入れ替えると (1), (2) の式が入れ替わるから、これだけを考えればよい。

これをよく見ると、$\text{min}\{ f(s, a) + \text{const} + f(t, b) \mid 1 \leq s \leq t \leq 10^9\}$ のような形になっている。
（ $f(x,y) = \sum_{1 \leq i \leq n} \lvert x - y_i \rvert$ みたいな。）
1,2 項目の min を与える $s,t$ を個別に計算した結果 $s \leq t$ が成り立っていればそのまま最小値にしてしまってよいが、 $f(x, \text{fixed \(y\)})$ の最小値は $x$ が $y$ の中央値のときであるので、 $a_i \leq b_i$ の制限によりそのまま最小値を計算してしまってよい。

結論として、$s,t$ を $\{a_i\}, \{b_i\}$ の中央値としたときの
$\sum_{1 \leq i \leq n} \lvert s - a_i \rvert + \sum_{1 \leq i \leq n} \lvert a_i - b_i \rvert + \sum_{1 \leq i \leq n} \lvert t - b_i \rvert$ が最小値である。
## 2
ここまでしなくても、 $(s,t)$ に関する関数として $\sum_{1 \leq i \leq n} \lvert s - a_i \rvert + \lvert a_i - b_i \rvert + \lvert b_i - t \rvert$ の傾き（離散な関数なのであまり正しい言葉づかいではない）が変化しうるのは明らかに $s,t$ が $a_i, b_i$ のときなので、このうちの最小値を求めればよい。

# 実装のメモ
## 1
```Rust
fn min_of(a: &[(usize, usize)]) -> usize {
    let s_min: usize = {
        let mut vec: Vec<usize> = a.into_iter()
            .map(|(x,_)|{*x}).collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().map(|item|{item.abs_diff(mid)}).sum()
    };
    let m_cst: usize = {
        a.into_iter().map(|(x, y)|{(*x).abs_diff(*y)}).sum()
    };
    let t_min: usize = {
        let mut vec: Vec<usize> = a.into_iter()
            .map(|(_, y)|{*y}).collect();
        vec.sort();
        let mid = vec[vec.len() / 2];
        vec.into_iter().map(|item|{item.abs_diff(mid)}).sum()
    };
    s_min + m_cst + t_min
}
```
かなり素直な実装だと思う。
エラーがないのは
- `vec[vec.len() / 2]` のアクセスは $1 \leq n$ より `a` が空でないから `vec` が空でないので良い。
正しさについてはよい。

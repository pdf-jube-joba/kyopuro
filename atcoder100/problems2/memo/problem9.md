# 問題
前提
- $m$: $\mathbb{N} \times \mathbb{N}$ の集合でそれぞれの元が異なる
- $n$: $\mathbb{N} \times \mathbb{N}$ の集合でそれぞれの元が異なる
---
求めるもの
- $x$, $y$: 整数であって、 $m$ をそれぞれ $(x,y)$ だけ平行移動すると、$n$ に含まれる。

制約
- そのような $x$, $y$ は一意に定まるような $m$, $n$ のみを与える。
- $1 \leq m \leq 200$
- $1 \leq n \leq 1000$

# 回答 1
## 方針
$m$ の元を適当にとって $m_0$ とする。
解を与える $(x,y)$ はある $i \in n$ により $i - m_0$ と必ずあらわされる。なぜなら、 $m_0$ が $(x,y)$ だけ移動させると $i$ に含まれるため。
逆に考えると、 $i \in n$ ごとに $p = i - m_0$ を計算して、 $m$ をすべて $p$ だけ動かした $m^\prime$ が $n$ に含まれるかを判定すればよい。

## 実装
```Rust
fn vec_par(ms: &[(i32, i32)], ns: &[(i32, i32)]) -> (i32, i32) {
    vec_par_vec(ms, ns)[0]
}

fn vec_par_vec(ms: &[(i32, i32)], ns: &[(i32, i32)]) -> Vec<(i32, i32)> {
    ns.iter().filter_map(|x|{
        let par = (x.0 - ms[0].0 , x.1 - ms[0].1);
        let ms2: Vec<(i32, i32)> = ms.iter().map(|y|{
            (y.0 + par.0, y.1 + par.1)
        }).collect();
        let result = ms2.into_iter().all(|y|{
            ns.into_iter().any(|z|{y.0 == z.0 && y.1 == z.1})
        });
        if result {Some(par)} else {None}
    }).collect()
}
```

## 評価
- `contains` 使うべきところを `into_iter().any()` にしてる。
- `ms2` を collect したのちに into_iter() しているため無駄にメモリを使うなどする。
- 解が見つかった時点で探索を打ち切ったほうが早いため処理がそうなるように書いたほうが良い。

## 計算量
長さ $\lvert n \rvert$ の iterator を `filter_map` し、 $\lvert m \rvert$ 分の配列を処理したのちに、 $m$ の各元について $n$ に含まれているか判定するため内部の処理は $O(\lvert m \rvert \lvert n \rvert)$ かかる。
そのため、 $O(\lvert m \rvert \lvert n \rvert^2)$ のようになる。

# 回答 2
## 方針
同じく。
## 実装
```Rust
fn vec_par(ms: &[(i32, i32)], ns: &[(i32, i32)]) -> (i32, i32) {
    ns.iter().filter_map(|x|{
        let par = (x.0 - ms[0].0 , x.1 - ms[0].1);
        let result = ms.iter()
            .map(|y| (y.0 + par.0, y.1 + par.1))
            .all(|y| ns.contains(&y));
        if result {Some(par)} else {None}
    }).next().unwrap()
}
```
## 評価
とりあえず内部を改善しただけ
- `unwrap` の正当性は回答における制約からくる。